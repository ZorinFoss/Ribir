use ribir_core::prelude::*;

use crate::prelude::*;

/// Avatar usage
///
/// # Example
///
/// ```
/// # use ribir_core::prelude::*;
/// # use ribir_widgets::prelude::*;
///
/// fn_widget! {
///   @ Avatar {
///     @ { Label::new("A") }
///   }
/// };
///
/// # #[cfg(feature="png")]
/// fn_widget! {
///   @ Avatar {
///     @ { ShallowImage::from_png(include_bytes!("../../gpu/examples/leaves.png")) }
///   }
/// };
/// ```
#[derive(Declare, Default, Clone)]
pub struct Avatar {
  #[declare(default=Palette::of(BuildCtx::get()).primary())]
  pub color: Color,
}

#[derive(Clone)]
pub struct AvatarStyle {
  pub size: Size,
  pub radius: Option<f32>,
  pub text_style: TextStyle,
}

impl CustomStyle for AvatarStyle {
  fn default_style(ctx: &impl AsRef<ProviderCtx>) -> Self {
    AvatarStyle {
      size: Size::splat(40.),
      radius: Some(20.),
      text_style: TypographyTheme::of(ctx).body_large.text.clone(),
    }
  }
}

pub struct AvatarDecorator;

impl ComposeDecorator for AvatarDecorator {
  fn compose_decorator(_: State<Self>, host: Widget) -> Widget { host }
}

#[derive(Template)]
pub enum AvatarTemplate {
  Text(Label),
  Image(Resource<PixelImage>),
}

impl ComposeChild<'static> for Avatar {
  type Child = AvatarTemplate;

  fn compose_child(this: impl StateWriter<Value = Self>, child: Self::Child) -> Widget<'static> {
    fn_widget! {
      let ctx = BuildCtx::get();
      let AvatarStyle { size, radius, text_style } = AvatarStyle::of(ctx);
      let palette1 = Palette::of(ctx).clone();
      let palette2 = Palette::of(ctx).clone();
      let w: Widget = match child {
        AvatarTemplate::Text(text) => {
          let mut container = @Container { size };
          if let Some(radius) = radius {
            container = container.radius(Radius::all(radius));
          }
          @ $container {
            background: pipe!(Brush::from(palette1.base_of(&$this.color))),
            clip_boundary: true,
            @Text {
              h_align: HAlign::Center,
              v_align: VAlign::Center,
              text: text.0,
              text_style,
              foreground: pipe!(Brush::from(palette2.on_of(&palette2.base_of(&$this.color)))),
            }
          }.into_widget()
        },
        AvatarTemplate::Image(image) => {
          let image = FatObj::new(image);
          let clip = radius.map(|radius| {
            let path = Path::rect_round(
              &Rect::from_size(size),
              &Radius::all(radius),
            );
            Clip { clip_path: path }
          });
          @$clip {
            @Container {
              size,
              @$image {
                box_fit: BoxFit::Contain,
              }
            }
          }
        }
      };

      @SizedBox {
        size,
        @ { w }
      }
    }
    .into_widget()
  }
}
