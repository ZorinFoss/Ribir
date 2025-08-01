use proc_macro2::TokenStream;
use quote::quote;

use crate::{
  error::result_to_token_stream,
  symbol_process::{DollarRefsCtx, symbol_to_macro},
  watch_macro::BodyExpr,
};

pub(crate) fn gen_code(input: TokenStream, ctx: Option<&mut DollarRefsCtx>) -> TokenStream {
  let res = symbol_to_macro(input).and_then(|input| {
    let body = syn::parse2::<BodyExpr>(input)?;
    let (stmts, refs) = if let Some(ctx) = ctx {
      ctx.new_dollar_scope(None);
      let stmts = body.fold(ctx).0;
      let refs = ctx.pop_dollar_scope(false);
      (stmts, refs)
    } else {
      let mut ctx = DollarRefsCtx::top_level();
      let stmts = body.fold(&mut ctx).0;
      let refs = ctx.pop_dollar_scope(false);

      (stmts, refs)
    };
    if !refs.is_empty() {
      Ok(quote! {{
        #refs
        let f = move || { #(#stmts)* };
        FnWidget::new(f)
      }})
    } else {
      Ok(quote! {{
          let f = move || { #(#stmts)* };
          FnWidget::new(f)
      }})
    }
  });

  result_to_token_stream(res)
}
