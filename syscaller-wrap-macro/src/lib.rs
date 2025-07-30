use proc_macro::TokenStream;
use quote::quote;
use syn::parenthesized;
use syn::{
    Ident, LitInt, Token, Type, parse::Parse, parse_macro_input, parse_quote,
    punctuated::Punctuated,
};

struct SyscallInputs(Vec<SyscallInput>);

impl Parse for SyscallInputs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let defs: Punctuated<SyscallInput, Token![,]> = Punctuated::parse_terminated(input)?;
        Ok(SyscallInputs(defs.into_iter().collect()))
    }
}

#[proc_macro]
pub fn wrap_syscall(input: TokenStream) -> TokenStream {
    let syscall_defs = parse_macro_input!(input as SyscallInputs);
    let generated: Vec<_> = syscall_defs
        .0
        .into_iter()
        .map(generate_syscall_wrapper)
        .collect();
    quote! {
        #(#generated)*
    }
    .into()
}

struct SyscallInput {
    pub syscall_number: u32,
    pub return_type: CType,
    pub name: Ident,
    pub args: Vec<CParameter>,
}

impl SyscallInput {}

impl Parse for SyscallInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let syscall_number: LitInt = input.parse()?;
        let _: Token![:] = input.parse()?;
        let return_type: CType = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        let _ = parenthesized!(content in input);
        let args: Punctuated<CParameter, Token![,]> =
            content.parse_terminated(CParameter::parse, Token![,])?;
        Ok(SyscallInput {
            syscall_number: syscall_number.base10_parse::<u32>()?,
            return_type,
            name,
            args: args.into_iter().collect(),
        })
    }
}

#[derive(Clone, Debug)]
enum CType {
    Base(BaseType),
    Pointer { target: Box<CType>, is_const: bool },
}

#[derive(Clone, Debug)]
enum BaseType {
    // Standard integer types
    Char,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    LongLong,
    UnsignedLongLong,
    Short,
    UnsignedShort,

    // Size types
    SizeT,
    SSizeT,

    // Special types
    Void,

    // Custom types (struct foo, union bar, etc.)
    #[allow(dead_code)]
    Custom(String),
}

impl BaseType {
    fn to_rust_type(&self) -> Type {
        match self {
            BaseType::Char => parse_quote!(i8),
            BaseType::Int => parse_quote!(i32),
            BaseType::UnsignedInt => parse_quote!(u32),
            BaseType::Long => parse_quote!(i64),
            BaseType::UnsignedLong => parse_quote!(u64),
            BaseType::LongLong => parse_quote!(i64),
            BaseType::UnsignedLongLong => parse_quote!(u64),
            BaseType::Short => parse_quote!(i16),
            BaseType::UnsignedShort => parse_quote!(u16),
            BaseType::SizeT => parse_quote!(usize),
            BaseType::SSizeT => parse_quote!(isize),
            BaseType::Void => parse_quote!(()),
            BaseType::Custom(_) => {
                // For custom types in syscalls, we typically use usize
                // since syscalls don't preserve high-level type information
                parse_quote!(usize)
            }
        }
    }

    fn to_syscall_arg(&self, arg_name: &Ident) -> proc_macro2::TokenStream {
        match self {
            BaseType::SizeT => quote!(#arg_name),
            BaseType::SSizeT => quote!(#arg_name as usize),
            _ => quote!(#arg_name as usize),
        }
    }
}

fn parse_base_type(input: syn::parse::ParseStream) -> syn::Result<BaseType> {
    let mut type_words = Vec::new();

    // collect type words until we hit * or identifier (param name)
    while input.peek(Ident) {
        let fork = input.fork();
        let ident: Ident = fork.parse()?;
        let word = ident.to_string();

        // check if this is a type keyword
        if matches!(
            word.as_str(),
            "char"
                | "short"
                | "int"
                | "long"
                | "void"
                | "size_t"
                | "ssize_t"
                | "unsigned"
                | "struct"
                | "union"
                | "enum"
        ) {
            input.parse::<Ident>()?; // consume the token
            type_words.push(word.clone());
            if matches!(word.as_str(), "struct" | "union" | "enum")
                && input.peek(Ident)
                && !input.peek2(Token![*])
                && !looks_like_param_name(input)
            {
                let name: Ident = input.parse()?;
                type_words.push(name.to_string());
            }
        } else {
            // this is not a type keyword so it should be the parameter name, stop parsing type
            break;
        }
    }
    let type_str = type_words.join(" ");
    match type_str.as_str() {
        "char" => Ok(BaseType::Char),
        "int" | "signed int" => Ok(BaseType::Int),
        "unsigned int" | "unsigned" => Ok(BaseType::UnsignedInt),
        "long" | "long int" | "signed long" => Ok(BaseType::Long),
        "unsigned long" | "unsigned long int" => Ok(BaseType::UnsignedLong),
        "long long" | "long long int" => Ok(BaseType::LongLong),
        "unsigned long long" | "unsigned long long int" => Ok(BaseType::UnsignedLongLong),
        "short" | "short int" => Ok(BaseType::Short),
        "unsigned short" | "unsigned short int" => Ok(BaseType::UnsignedShort),
        "void" => Ok(BaseType::Void),
        "size_t" => Ok(BaseType::SizeT),
        "ssize_t" => Ok(BaseType::SSizeT),
        _ if !type_str.is_empty() => Ok(BaseType::Custom(type_str)),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected type specifier",
        )),
    }
}

fn looks_like_param_name(input: syn::parse::ParseStream) -> bool {
    if let Ok(ident) = input.fork().parse::<Ident>() {
        let word = ident.to_string();
        // Parameter names typically don't start with known type keywords
        !matches!(
            word.as_str(),
            "const"
                | "char"
                | "int"
                | "unsigned"
                | "signed"
                | "long"
                | "short"
                | "void"
                | "size_t"
                | "ssize_t"
                | "struct"
                | "union"
                | "enum"
        )
    } else {
        true
    }
}

impl CType {
    /// Convert C type to equivalent Rust type
    ///
    /// # Security Note
    /// These mappings assume Linux x86_64 ABI. For cross-platform code,
    /// consider using libc crate types or cfg-conditional compilation.
    fn to_rust_type(&self) -> Type {
        match self {
            CType::Base(base) => base.to_rust_type(),
            CType::Pointer { is_const, target } => {
                // All const pointers become *const u8 for syscalls
                // This provides type erasure while preserving const semantics
                if matches!(**target, CType::Base(BaseType::Char)) {
                    parse_quote!(impl AsRef<[u8]>)
                } else if *is_const {
                    parse_quote!(*const u8)
                } else {
                    parse_quote!(*mut u8)
                }
            }
        }
    }

    /// Generate type conversion for syscall argument
    ///
    /// # Vulnerability Mitigation
    /// Explicit casts prevent silent truncation that could lead to
    /// security vulnerabilities in syscall arguments.
    fn to_syscall_arg(&self, arg_name: &Ident) -> proc_macro2::TokenStream {
        match self {
            CType::Base(base) => base.to_syscall_arg(arg_name),
            CType::Pointer { target, .. } => {
                if matches!(**target, CType::Base(BaseType::Char)) {
                    quote!(#arg_name.as_ref().as_ptr() as usize)
                } else {
                    quote!(#arg_name as usize)
                }
            }
        }
    }
}

impl Parse for CType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse initial const qualifier
        let mut is_const = false;
        if input.peek(Token![const]) {
            input.parse::<Token![const]>()?;
            is_const = true;
        }

        let base_type = parse_base_type(input)?;
        let mut current_type = CType::Base(base_type);

        // Parse pointer levels from right to left
        // int ** becomes Pointer -> Pointer -> int
        let mut pointer_levels = Vec::new();

        while input.peek(Token![*]) {
            input.parse::<Token![*]>()?;

            // Check for const after *
            let ptr_const = if input.peek(Token![const]) {
                input.parse::<Token![const]>()?; // consume const
                true
            } else {
                false
            };

            pointer_levels.push(ptr_const || is_const);
            is_const = false; // const only applies to first pointer level if at beginning
        }
        for &ptr_is_const in pointer_levels.iter() {
            current_type = CType::Pointer {
                target: Box::new(current_type),
                is_const: ptr_is_const,
            };
        }

        Ok(current_type)
    }
}

struct CParameter {
    pub c_type: CType,
    pub name: Ident,
}

impl Parse for CParameter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let c_type: CType = input.parse()?;
        let name: Ident = input.parse()?;
        Ok(CParameter { c_type, name })
    }
}
/// Generate the complete syscall wrapper function
fn generate_syscall_wrapper(def: SyscallInput) -> proc_macro2::TokenStream {
    let func_name = &def.name;

    // Generate parameter list for function signature
    let rust_params: Vec<_> = def
        .args
        .iter()
        .map(|param| {
            let name = &param.name;
            let rust_type = param.c_type.to_rust_type();
            quote!(#name: #rust_type)
        })
        .collect();

    // Generate syscall arguments
    let syscall_args: Vec<_> = def
        .args
        .iter()
        .map(|param| param.c_type.to_syscall_arg(&param.name))
        .collect();

    // Determine appropriate syscall function based on argument count
    let syscall_func = match def.args.len() {
        0 => quote!(syscall0),
        1 => quote!(syscall1),
        2 => quote!(syscall2),
        3 => quote!(syscall3),
        4 => quote!(syscall4),
        5 => quote!(syscall5),
        6 => quote!(syscall6),
        _ => {
            return quote! {
                compile_error!("Syscalls with more than 6 arguments are not supported");
            };
        }
    };

    let return_type = def.return_type.to_rust_type();
    let sysno = def.syscall_number as usize;

    // Generate the complete function
    quote! {
        /// Generated syscall wrapper
        ///
        /// # Safety
        /// This function is unsafe because it directly invokes system calls,
        /// bypassing Rust's memory safety guarantees. Callers must ensure:
        /// - All pointer arguments point to valid memory
        /// - Buffer sizes are correct and within bounds
        /// - The syscall contract is properly understood and followed
        ///
        /// # Security Considerations
        /// - Input validation should be performed by the caller
        /// - Consider TOCTTOU (Time-of-Check-Time-of-Use) race conditions
        /// - Be aware of potential privilege escalation through syscall fuzzing
        #[inline(always)]
        pub unsafe fn #func_name(#(#rust_params),*) -> #return_type {
            unsafe {
                syscaller::#syscall_func(#sysno, #(#syscall_args),*) as #return_type
            }
        }
    }
}
