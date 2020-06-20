//! Contour plot

use crate::common::color::Color;
use crate::common::{Calendar, ColorBar, ColorScale, Dim, Font, HoverInfo, Label, Line, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ContoursType {
    #[serde(rename = "levels")]
    Levels,
    #[serde(rename = "constraint")]
    Constraint,
}

#[derive(Serialize, Debug)]
pub enum ContoursColoring {
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "heatmap")]
    HeatMap,
    #[serde(rename = "lines")]
    Lines,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Debug)]
pub struct Contours {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<ContoursType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coloring: Option<ContoursColoring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlabels")]
    show_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "labelfont")]
    label_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "labelformat")]
    label_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
}

impl Contours {
    pub fn new() -> Contours {
        Contours {
            r#type: None,
            start: None,
            end: None,
            size: None,
            coloring: None,
            show_lines: None,
            show_labels: None,
            label_font: None,
            label_format: None,
            operation: None,
            value: None,
        }
    }

    pub fn type_(mut self, t: ContoursType) -> Contours {
        self.r#type = Some(t);
        self
    }

    pub fn start(mut self, start: f64) -> Contours {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: f64) -> Contours {
        self.end = Some(end);
        self
    }

    pub fn size(mut self, size: usize) -> Contours {
        self.size = Some(size);
        self
    }

    pub fn coloring(mut self, coloring: ContoursColoring) -> Contours {
        self.coloring = Some(coloring);
        self
    }

    pub fn show_lines(mut self, show_lines: bool) -> Contours {
        self.show_lines = Some(show_lines);
        self
    }

    pub fn show_labels(mut self, show_labels: bool) -> Contours {
        self.show_labels = Some(show_labels);
        self
    }

    pub fn label_font(mut self, label_font: Font) -> Contours {
        self.label_font = Some(label_font);
        self
    }

    pub fn label_format(mut self, label_format: &str) -> Contours {
        self.label_format = Some(label_format.to_owned());
        self
    }

    pub fn operation(mut self, operation: &str) -> Contours {
        self.operation = Some(operation.to_owned());
        self
    }

    pub fn value(mut self, value: f64) -> Contours {
        self.value = Some(value);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Contour<Z, X = f64, Y = f64>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    x0: Option<X>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dx: Option<X>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    y0: Option<Y>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dy: Option<Y>,

    z: Vec<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zhoverformat")]
    zhover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmax: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmid: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmin: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocontour")]
    auto_contour: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contours: Option<Contours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_contours: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<Z> Contour<Z, f64, f64>
where
    Z: Serialize,
{
    pub fn new_z(z: Vec<Z>) -> Contour<Z, f64, f64> {
        Contour {
            r#type: PlotType::Contour,
            x: None,
            x0: None,
            dx: None,
            y: None,
            y0: None,
            dy: None,
            z,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            line: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            auto_contour: None,
            connect_gaps: None,
            contours: None,
            fill_color: None,
            hover_label: None,
            hover_on_gaps: None,
            n_contours: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<Z, X, Y> Contour<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Contour<Z, X, Y> {
        Contour {
            r#type: PlotType::Contour,
            x: Some(x),
            x0: None,
            dx: None,
            y: Some(y),
            y0: None,
            dy: None,
            z,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            line: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            auto_contour: None,
            connect_gaps: None,
            contours: None,
            fill_color: None,
            hover_label: None,
            hover_on_gaps: None,
            n_contours: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        }
    }

    pub fn x(mut self, x: Vec<X>) -> Contour<Z, X, Y> {
        self.x = Some(x);
        self
    }

    pub fn x0(mut self, x0: X) -> Contour<Z, X, Y> {
        self.x0 = Some(x0);
        self
    }

    pub fn dx(mut self, dx: X) -> Contour<Z, X, Y> {
        self.dx = Some(dx);
        self
    }

    pub fn y0(mut self, y0: Y) -> Contour<Z, X, Y> {
        self.y0 = Some(y0);
        self
    }

    pub fn dy(mut self, dy: Y) -> Contour<Z, X, Y> {
        self.dy = Some(dy);
        self
    }

    pub fn y(mut self, y: Vec<Y>) -> Contour<Z, X, Y> {
        self.y = Some(y);
        self
    }

    pub fn name(mut self, name: &str) -> Contour<Z, X, Y> {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(mut self, visible: bool) -> Contour<Z, X, Y> {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> Contour<Z, X, Y> {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(mut self, legend_group: &str) -> Contour<Z, X, Y> {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Contour<Z, X, Y> {
        self.opacity = Some(opacity);
        self
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> Contour<Z, X, Y> {
        let text = private::owned_string_vector(text);
        self.text = Some(text);
        self
    }

    pub fn hover_text(mut self, hover_text: Vec<String>) -> Contour<Z, X, Y> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Contour<Z, X, Y> {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> Contour<Z, X, Y> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        self
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Contour<Z, X, Y> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        self
    }

    pub fn line(mut self, line: Line) -> Contour<Z, X, Y> {
        self.line = Some(line);
        self
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Contour<Z, X, Y> {
        self.color_bar = Some(color_bar);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Contour<Z, X, Y> {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Contour<Z, X, Y> {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> Contour<Z, X, Y> {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Contour<Z, X, Y> {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn zauto(mut self, zauto: bool) -> Contour<Z, X, Y> {
        self.zauto = Some(zauto);
        self
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> Contour<Z, X, Y> {
        self.zhover_format = Some(zhover_format.to_owned());
        self
    }

    pub fn zmax(mut self, zmax: Z) -> Contour<Z, X, Y> {
        self.zmax = Some(zmax);
        self
    }

    pub fn zmid(mut self, zmid: Z) -> Contour<Z, X, Y> {
        self.zmid = Some(zmid);
        self
    }

    pub fn zmin(mut self, zmin: Z) -> Contour<Z, X, Y> {
        self.zmin = Some(zmin);
        self
    }

    pub fn auto_contour(mut self, auto_contour: bool) -> Contour<Z, X, Y> {
        self.auto_contour = Some(auto_contour);
        self
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Contour<Z, X, Y> {
        self.connect_gaps = Some(connect_gaps);
        self
    }

    pub fn contours(mut self, contours: Contours) -> Contour<Z, X, Y> {
        self.contours = Some(contours);
        self
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Contour<Z, X, Y> {
        self.fill_color = Some(fill_color.to_color_string());
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> Contour<Z, X, Y> {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> Contour<Z, X, Y> {
        self.hover_on_gaps = Some(hover_on_gaps);
        self
    }

    pub fn n_contours(mut self, n_contours: usize) -> Contour<Z, X, Y> {
        self.n_contours = Some(n_contours);
        self
    }

    pub fn transpose(mut self, transpose: bool) -> Contour<Z, X, Y> {
        self.transpose = Some(transpose);
        self
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Contour<Z, X, Y> {
        self.x_calendar = Some(x_calendar);
        self
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Contour<Z, X, Y> {
        self.y_calendar = Some(y_calendar);
        self
    }
}

impl<X, Y, Z> Trace for Contour<X, Y, Z>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
