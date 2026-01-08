use syn::{LitStr, Type, Path, Token};
use syn::parse::{Parse, ParseStream};
use proc_macro2::Span;
use syn::spanned::Spanned;

/// A source type to generate tests from.
///
/// # Variants
/// - `SourceType::JsonFile(LitStr, Type, Span)` — pass a path to a JSON file and a type
/// to deserialize it into.`
#[allow(dead_code)]
pub enum SourceType {
    JsonFile(LitStr, Option<Type>, Span),
    JsonString(LitStr, Option<Type>, Span),
    JsonResponse(LitStr, Option<Type>, Span),
    PathMask(LitStr, Span),
}

impl SourceType {
    pub fn span(&self) -> Span {
        match self {
            SourceType::JsonFile(_, _, span) => *span,
            SourceType::JsonString(_, _, span) => *span,
            SourceType::JsonResponse(_, _, span) => *span,
            SourceType::PathMask(_, span) => *span,
        }
    }
}

impl Parse for SourceType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 1. Parse the path (e.g., SourceType::<User>::JsonFile or JsonFile)
        let path: Path = input.parse()?;
        let path_span = path.span();

        // 2. Extract type from turbofish if present in any segment
        let mut generic_type: Option<Type> = None;
        for segment in &path.segments {
            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if args.args.is_empty() {
                    return Err(syn::Error::new_spanned(args, "Expected exactly one type argument in turbofish"));
                }
                if args.args.len() > 1 {
                    return Err(syn::Error::new_spanned(args, "Expected exactly one type argument in turbofish"));
                }
                if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                    generic_type = Some(ty.clone());
                    break;
                } else {
                    return Err(syn::Error::new_spanned(args, "Expected a type argument in turbofish"));
                }
            }
        }

        // 3. Identify the variant
        let last_segment = path.segments.last()
            .ok_or_else(|| syn::Error::new_spanned(&path, "Expected a variant name"))?;
        
        match last_segment.ident.to_string().as_str() {
            "JsonFile" => {
                let content;
                syn::parenthesized!(content in input);
                
                // Parse the path (Required)
                let file_path: LitStr = content.parse()?;
                
                // Parse the type if it follows a comma: ("path", User)
                let mut arg_type: Option<Type> = None;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    arg_type = Some(content.parse()?);
                }

                // Preference: argument type > turbofish type
                let final_type = arg_type.or(generic_type);

                Ok(SourceType::JsonFile(file_path, final_type, path_span))
            }
            "JsonString" => {
                let content;
                syn::parenthesized!(content in input);
                
                // Parse the JSON string (Required)
                let json_string: LitStr = content.parse()?;
                
                // Parse the type if it follows a comma: ("{}", User)
                let mut arg_type: Option<Type> = None;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    arg_type = Some(content.parse()?);
                }

                // Preference: argument type > turbofish type
                let final_type = arg_type.or(generic_type);

                Ok(SourceType::JsonString(json_string, final_type, path_span))
            }
            "JsonResponse" => {
                let content;
                syn::parenthesized!(content in input);

                // Parse the URL (Required)
                let url: LitStr = content.parse()?;

                // Parse the type if it follows a comma: ("url", User)
                let mut arg_type: Option<Type> = None;
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                    arg_type = Some(content.parse()?);
                }

                // Preference: argument type > turbofish type
                let final_type = arg_type.or(generic_type);

                Ok(SourceType::JsonResponse(url, final_type, path_span))
            }
            "PathMask" => {
                let content;
                syn::parenthesized!(content in input);

                // Parse the path (Required)
                let path_mask: LitStr = content.parse()?;

                Ok(SourceType::PathMask(path_mask, path_span))
            }
            v => Err(syn::Error::new_spanned(last_segment, format!("Unknown variant: {}", v))),
        }
    }
}
