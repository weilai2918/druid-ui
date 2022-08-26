use druid::widget::{ControllerHost,Click};
use druid::widget::prelude::*;
use druid::Color;
use tracing::{instrument, trace};
use druid::{theme, Affine, Data, Insets, LinearGradient, UnitPoint};
use crate::widget::label::{Label,LabelText};


const LABEL_INSETS: Insets = Insets::uniform_xy(8., 2.);

pub struct Button<T> {
    label: Label<T>,
    label_size: Size,
    style: ButtonStyle
}


pub enum ButtonStyle{
    ORANGE,
    RED,
    YELLOW,
    BLUE,
    PURPLE,
    GREEN,
    NONE
}

#[derive(Clone)]
struct ButtonStyleColor{
    border_color:Color,
    background_color:Color,
    active_border_color:Color,
    active_background_color:Color,
    hot_border_color:Color,
    hot_background_color:Color,
    text_color:Color
}

impl ButtonStyleColor {

    fn get_style(style: &ButtonStyle) -> Option<Self> {
        match style {
            ButtonStyle::RED => {
                Some(Self{
                    border_color:Color::rgb8(255,105,94),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(255,105,94),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(255,105,94),
                    hot_background_color:Color::rgb8(255,105,94),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            ButtonStyle::ORANGE => {
                Some(Self{
                    border_color:Color::rgb8(255,133,27),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(255,133,27),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(255,133,27),
                    hot_background_color:Color::rgb8(255,133,27),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            ButtonStyle::YELLOW => {
                Some(Self{
                    border_color:Color::rgb8(255,226,31),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(255,226,31),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(255,226,31),
                    hot_background_color:Color::rgb8(255,226,31),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            ButtonStyle::BLUE => {
                Some(Self{
                    border_color:Color::rgb8(84,200,255),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(84,200,255),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(84,200,255),
                    hot_background_color:Color::rgb8(84,200,255),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            ButtonStyle::PURPLE => {
                Some(Self{
                    border_color:Color::rgb8(220,115,255),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(220,115,255),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(220,115,255),
                    hot_background_color:Color::rgb8(220,115,255),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            ButtonStyle::GREEN => {
                Some(Self{
                    border_color:Color::rgb8(46,204,64),
                    background_color:Color::rgb8(0,0,0),
                    active_border_color:Color::rgb8(46,204,64),
                    active_background_color:Color::rgb8(0,0,0),
                    hot_border_color:Color::rgb8(46,204,64),
                    hot_background_color:Color::rgb8(46,204,64),
                    text_color:Color::rgb8(255,255,255)
                })
            }
            _ => {
                None
            }
        }
    }

}


impl<T: Data> Button<T> {

    pub fn new(text: impl Into<LabelText<T>>,style:ButtonStyle) -> Button<T> {
        let button_style = ButtonStyleColor::get_style(&style);
        if let Some(color) = button_style{
            Button::from_label(Label::with_and_color(text,color.text_color),style)
        }else{
            Button::from_label(Label::with_and_color(text,Color::rgb8(255,255,255)),style)
        }
    }


    pub fn from_label(label: Label<T>,style:ButtonStyle) -> Button<T> {
        Button {
            label,
            label_size: Size::ZERO,
            style:style
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        Button::new(text,ButtonStyle::RED)
    }

    
    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }

}


impl<T: Data> Widget<T> for Button<T> {
    #[instrument(name = "Button", level = "trace", skip(self, ctx, event, _data, _env))]
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                if !ctx.is_disabled() {
                    ctx.set_active(true);
                    ctx.request_paint();
                    trace!("Button {:?} pressed", ctx.widget_id());
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() && !ctx.is_disabled() {
                    ctx.request_paint();
                    trace!("Button {:?} released", ctx.widget_id());
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    #[instrument(name = "Button", level = "trace", skip(self, ctx, event, data, env))]
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) | LifeCycle::DisabledChanged(_) = event {
            ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    #[instrument(name = "Button", level = "trace", skip(self, ctx, old_data, data, env))]
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    #[instrument(name = "Button", level = "trace", skip(self, ctx, bc, data, env))]
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Button");
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width+15.,
            (self.label_size.height + padding.height+15.).max(min_height),
        ));
        trace!("Computed button size: {}", button_size);
        button_size
    }

    #[instrument(name = "Button", level = "trace", skip(self, ctx, data, env))]
    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_active = ctx.is_active() && !ctx.is_disabled();
        
        let is_hot = ctx.is_hot();
        let size = ctx.size();
    
        //默认宽度
        let stroke_width = 5.0;
        //let stroke_width = env.get(theme::BUTTON_BORDER_WIDTH);

        let button_style = ButtonStyleColor::get_style(&self.style);

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let bg_gradient = if ctx.is_disabled() {
            //禁用状态
            LinearGradient::new(
                UnitPoint::TOP,
                UnitPoint::BOTTOM,
                (
                    env.get(theme::DISABLED_BUTTON_LIGHT),
                    env.get(theme::DISABLED_BUTTON_DARK),
                ),
            )
        } else if is_active {
            //点击
            let style = button_style;
            if let Some(color) = style {
                let color_top = color.active_background_color.clone();
                let color_bottom = color.active_background_color;
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (color_top, color_bottom),
                )
            }else{
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
                )
            }
        } else if is_hot {
            let style = button_style;
            if let Some(color) = style {
                let color_top = color.hot_background_color.clone();
                let color_bottom = color.hot_background_color;
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (color_top, color_bottom),
                )
            }else{
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
                )
            }
        } else {
            let style = button_style;
            if let Some(color) = style {
                let color_top = color.background_color.clone();
                let color_bottom = color.background_color;
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (color_top, color_bottom),
                )
            }else{
                LinearGradient::new(
                    UnitPoint::TOP,
                    UnitPoint::BOTTOM,
                    (env.get(theme::BUTTON_DARK), env.get(theme::BUTTON_LIGHT)),
                )
            }
        };

        let button_style = ButtonStyleColor::get_style(&self.style);
        //鼠标悬浮
        let border_color = if is_hot && !ctx.is_disabled() {
            let style = button_style;
            if let Some(color) = style {
                color.hot_border_color
            }else{
                env.get(theme::BORDER_LIGHT)
            }
        } else if is_active && !ctx.is_disabled(){
            let style = button_style;
            if let Some(color) = style {
                color.active_border_color
            }else{
                env.get(theme::BORDER_LIGHT)
            }
        } else {
            let style = button_style;
            if let Some(color) = style {
                color.border_color
            }else{
                env.get(theme::BORDER_DARK)
            }
        };

        ctx.stroke(rounded_rect, &border_color, stroke_width);

        ctx.fill(rounded_rect, &bg_gradient);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}