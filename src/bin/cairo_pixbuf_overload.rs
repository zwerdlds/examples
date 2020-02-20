#![feature(allocator_api)]
extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate gdk;
extern crate glib;
extern crate opencv;
extern crate gdk_pixbuf;
use gdk::prelude::GdkContextExt;
use gdk_pixbuf::Colorspace::Rgb;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib::Bytes;
use gtk::Button;
use gtk::DrawingArea;
use gtk::prelude::*;
use opencv::imgproc::cvt_color;
use opencv::imgproc::COLOR_BGR2RGB;
use opencv::imgcodecs;
use opencv::imgcodecs::imread;
use std::env::args;

const TEST_FILE_PATH: &'static str = "/home/zwerdlds/Desktop/test.jpg";



fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.cairo_pixbuf_overload"),
        Default::default(),
    )
    .expect("Initialization failed...");

    let i = imread(
            TEST_FILE_PATH,
            imgcodecs::IMREAD_COLOR)
        .expect("OpenCV imread failed.");
    let mut o = i.clone().unwrap(); 
    cvt_color(&i, &mut o, COLOR_BGR2RGB, 0).expect("Convert to RGB");


    application.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);

        let box_area =  gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.add(&box_area);

        let reload_btn = Button::new_with_label("Queue Draw");
        let reload_btn_box_area = box_area.clone();
        box_area.pack_start(&reload_btn, false, false, 0);
        reload_btn.connect_clicked(move |_| {
            reload_btn_box_area.queue_draw();
        });


        let drawing_area = Box::new(DrawingArea::new)();
        box_area.pack_start(&drawing_area, true, true, 0);
        let draw_out_image = o.clone().unwrap();
        let rows_size = draw_out_image.rows().unwrap();
        let cols_size = draw_out_image.cols().unwrap();
        let step = draw_out_image.mat_step().unwrap().to_size_t().unwrap();
        let size = cols_size * rows_size * 3;

        drawing_area.connect_draw(move |_, cr| {
            let slice = /*unsafe {
//                let ptr = draw_out_image.ptr(0).unwrap() as *const u8 as *mut u8;
//                std::slice::from_raw_parts(ptr, size as usize).clone()
//            };*/
                vec![0; size as usize];

            let bytes = Bytes::from(&slice);

            let pixbuf = Pixbuf::new_from_bytes(
                &bytes, Rgb, false, 8,
                cols_size, rows_size,
                step as i32);

            cr.set_source_pixbuf(&pixbuf, 0., 0.);

            cr.paint();

            cr.set_source_rgb(0.,0.,0.);

            Inhibit(false)
        });

        window.set_default_size(500, 500);
        window.show_all();
    });

    application.run(&args().collect::<Vec<_>>());
}