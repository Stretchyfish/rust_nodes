use std::time::Duration;

use eframe::{self, Frame};
use egui::{self, Vec2};
mod node_graph_editor;

fn main() ->  Result<(), eframe::Error>
{   
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("app", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))))
}

#[derive(Default)]
struct MyEguiApp 
{
    nodes: Vec<node_graph_editor::nodes::Node>,
}

impl MyEguiApp 
{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self 
    {        
        Self::default()
    }
}

impl eframe::App for MyEguiApp 
{
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
   {
    //ctx.request_repaint();

    egui::TopBottomPanel::top("menu bar").show(ctx, |ui|
    {
        egui::menu::bar(ui, |ui|
        {
            ui.menu_button("settings", |ui|
            {
                if ui.button("screen settings").clicked()
                {
                    let new_node = node_graph_editor::nodes::Node::new();
                    self.nodes.push(new_node);
                }

                if ui.button("program settings").clicked()
                {
                    println!("program settings was clicked");
                }
            });

            if ui.button("add node").clicked()
            {
                let new_node = node_graph_editor::nodes::Node::new();
                self.nodes.push(new_node);
            };
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| 
    {
        for i in 0..self.nodes.len()
        {
            self.nodes[i].update(ui);
        }

        /*
        let mouse_pos_temp = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2{x: 0.0, y: 0.0});
        let new_rect = egui::Rect::from_min_size(mouse_pos_temp, egui::Vec2 {x: 100.0, y: 100.0});
        
        ui.painter().rect(new_rect, egui::Rounding::default(), egui::Color32::BLUE, egui::Stroke::new(2.0, egui::Color32::GRAY));
        */

        //ctx.request_repaint();
    });
    //ctx.request_repaint_after(Duration::new(1.0 / 60.0, 0.0 as u64));
   }
}

/*
struct TestSquare
{
    
}

impl egui::Widget for TestSquare
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response 
    {
        
        egui::Response::
    }
}
*/

