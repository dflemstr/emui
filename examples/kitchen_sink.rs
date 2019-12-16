use embedded_graphics::Drawing;
use emui::widgets;
use std::str;
use std::thread;
use std::time;

type FullColor = embedded_graphics::pixelcolor::Rgb888;
type FullFont<'a> = embedded_graphics::fonts::Font6x8<'a, FullColor>;
type MonochromeColor = embedded_graphics::pixelcolor::BinaryColor;
type MonochromeFont<'a> = embedded_graphics::fonts::Font6x8<'a, MonochromeColor>;

#[derive(Debug, structopt::StructOpt)]
struct Options {
    /// Which mode to run in ('rgb' or 'monochrome')
    mode: Mode,
}

#[derive(Debug)]
enum Mode {
    Rgb,
    Monochrome,
}

#[derive(Debug, Default)]
struct App {
    data_processing: DataProcessing,
    system_stats: SystemStats,

    main_container: widgets::container::State,
    frame: widgets::frame::State,
    ok_button: widgets::button::State,
}

#[derive(Debug, Default)]
struct DataProcessing {
    start: Option<time::Instant>,
    upload_ratio: f32,
    transcode_ratio: f32,

    container: widgets::container::State,
    upload_label: widgets::text::State,
    upload_progress: widgets::progress_bar::State,
    transcode_label: widgets::text::State,
    transcode_progress: widgets::progress_bar::State,
}

#[derive(Debug, Default)]
struct SystemStats {
    system: sysinfo::System,
    mem_ratio: f32,
    cpu_ratio: f32,

    container: widgets::container::State,
    cpu_label: widgets::text::State,
    cpu_progress: widgets::progress_bar::State,
    mem_label: widgets::text::State,
    mem_progress: widgets::progress_bar::State,
}

impl App {
    fn update(&mut self) {
        self.data_processing.update();
        self.system_stats.update();
    }

    fn view<'a, T>(&'a mut self) -> impl emui::widget::Node<'a, T>
    where
        T: emui::theme::Theme<'a>,
    {
        widgets::container::Main::new(
            &mut self.main_container,
            (
                widgets::frame::Frame::new(
                    &mut self.frame,
                    "Data processing",
                    self.data_processing.view(),
                ),
                self.system_stats.view(),
                widgets::button::Button::new(&mut self.ok_button, "OK"),
            ),
        )
    }
}

impl DataProcessing {
    fn update(&mut self) {
        let start = self.start.get_or_insert_with(time::Instant::now);
        let millis = start.elapsed().as_millis();

        self.upload_ratio = (millis as f32 / 4300.0) % 1.0;
        self.transcode_ratio = (millis as f32 / 9500.0) % 1.0;
    }

    fn view<'a, T>(&'a mut self) -> impl emui::widget::Node<'a, T>
    where
        T: emui::theme::Theme<'a>,
    {
        widgets::container::Column::new(
            &mut self.container,
            (
                widgets::text::Text::new(&mut self.upload_label, "Upload"),
                widgets::progress_bar::ProgressBar::new(
                    &mut self.upload_progress,
                    (self.upload_ratio * 1000.0) as u32,
                    1000,
                ),
                widgets::text::Text::new(&mut self.transcode_label, "Transcode"),
                widgets::progress_bar::ProgressBar::new(
                    &mut self.transcode_progress,
                    (self.transcode_ratio * 1000.0) as u32,
                    1000,
                ),
            ),
        )
    }
}

impl SystemStats {
    fn update(&mut self) {
        use sysinfo::ProcessorExt;
        use sysinfo::SystemExt;

        self.system.refresh_all();
        self.system.refresh_all();

        let processors = self.system.get_processor_list();
        let processor_usage_sum = processors.iter().map(|p| p.get_cpu_usage()).sum::<f32>();
        let processor_count = processors.len() as f32;

        self.cpu_ratio = processor_usage_sum / processor_count;
        self.mem_ratio =
            self.system.get_used_memory() as f32 / self.system.get_total_memory() as f32;
    }

    fn view<'a, T>(&'a mut self) -> impl emui::widget::Node<'a, T>
    where
        T: emui::theme::Theme<'a>,
    {
        widgets::container::Row::new(
            &mut self.container,
            (
                widgets::text::Text::new(&mut self.cpu_label, "CPU:"),
                widgets::progress_bar::ProgressBar::new(
                    &mut self.cpu_progress,
                    (self.cpu_ratio * 1000.0) as u32,
                    1000,
                ),
                widgets::text::Text::new(&mut self.mem_label, "MEM:"),
                widgets::progress_bar::ProgressBar::new(
                    &mut self.mem_progress,
                    (self.mem_ratio * 1000.0) as u32,
                    1000,
                ),
            ),
        )
    }
}

fn main() {
    use structopt::StructOpt;
    let options = Options::from_args();

    use embedded_graphics::pixelcolor::RgbColor;

    match options.mode {
        Mode::Rgb => {
            let theme = emui::theme::SimpleTheme::<FullColor, FullFont<'_>> {
                spacing: 2.0,
                background_color: embedded_graphics::pixelcolor::Rgb888::BLACK,
                text_color: embedded_graphics::pixelcolor::Rgb888::WHITE,
                border_color: embedded_graphics::pixelcolor::Rgb888::RED,
                fill_color: embedded_graphics::pixelcolor::Rgb888::BLUE,
                phantom: core::marker::PhantomData,
            };

            let display = embedded_graphics_simulator::DisplayBuilder::new()
                .size(320, 240)
                .scale(2)
                .build_rgb();

            let mut app = App::default();

            let mut emui = emui::Emui::new(display);

            loop {
                app.update();

                emui.drawing_mut().draw(embedded_graphics::egrectangle!(
                    (0, 0),
                    (320, 240),
                    fill = Some(embedded_graphics::pixelcolor::Rgb888::BLACK)
                ));

                emui.update(320, 240, &theme, &mut app.view());

                if emui.drawing_mut().run_once() {
                    break;
                }

                thread::sleep(time::Duration::from_millis(100));
            }
        }
        Mode::Monochrome => {
            let theme = emui::theme::SimpleTheme::<MonochromeColor, MonochromeFont<'_>> {
                spacing: 2.0,
                background_color: embedded_graphics::pixelcolor::BinaryColor::Off,
                text_color: embedded_graphics::pixelcolor::BinaryColor::On,
                border_color: embedded_graphics::pixelcolor::BinaryColor::On,
                fill_color: embedded_graphics::pixelcolor::BinaryColor::On,
                phantom: core::marker::PhantomData,
            };

            let display = embedded_graphics_simulator::DisplayBuilder::new()
                .size(160, 120)
                .scale(2)
                .theme(embedded_graphics_simulator::BinaryColorTheme::OledBlue)
                .build_binary();

            let mut app = App::default();

            let mut emui = emui::Emui::new(display);

            loop {
                app.update();

                emui.drawing_mut().draw(embedded_graphics::egrectangle!(
                    (0, 0),
                    (160, 120),
                    fill = Some(embedded_graphics::pixelcolor::BinaryColor::Off)
                ));

                emui.update(160, 120, &theme, &mut app.view());

                if emui.drawing_mut().run_once() {
                    break;
                }

                thread::sleep(time::Duration::from_millis(100));
            }
        }
    }
}

impl str::FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rgb" => Ok(Mode::Rgb),
            "monochrome" => Ok(Mode::Monochrome),
            _ => Err("must be either 'rgb' or 'monochrome'"),
        }
    }
}
