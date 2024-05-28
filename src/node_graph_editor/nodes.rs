use egui;


enum NodeType
{
    CommandLinePrinter,
}

pub struct Node
{
    title: String,
    node_rect: egui::Rect,
    is_moving: bool,
    grab_vector: egui::Vec2,
    node_title_color: egui::Color32,
    node_type: NodeType,
}

impl Node
{
    pub fn new() -> Self
    {
        Node
        {
            title: String::from("test"),
            node_rect: egui::Rect::from_min_size(egui::Pos2 {x: 100.0, y: 100.0}, egui::Vec2 {x: 200.0, y: 200.0}),
            is_moving: false,
            grab_vector: egui::Vec2 {x: 0.0, y: 0.0},
            node_title_color: egui::Color32::GRAY,
            node_type: NodeType::CommandLinePrinter,
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui)
    {
        let mouse_pos = ui.input(|i| i.pointer.hover_pos()).unwrap();
        let node_painter = ui.painter();

        // Move node
        if self.is_moving == true
        {
            self.node_rect = egui::Rect::from_min_size(egui::Pos2 {x: mouse_pos.x + self.grab_vector.x, y: mouse_pos.y + self.grab_vector.y}, egui::Vec2 {x: 200.0, y: 200.0});
        }

        // Draw outer rectangle
        node_painter.rect(self.node_rect, egui::Rounding::default(), egui::Color32::WHITE, egui::Stroke::new(2.0, egui::Color32::WHITE));

        // Draw title rectangle
        let node_title_rect = egui::Rect::from_min_size(self.node_rect.min, egui::Vec2::new(self.node_rect.size().x, 20.0));
        node_painter.rect(node_title_rect, egui::Rounding::default(), self.node_title_color, egui::Stroke::new(2.0, egui::Color32::GRAY));

        // Draw title text
        let title_pos =  egui::Pos2 { x: node_title_rect.min.x + node_title_rect.size().x / 2.0, y: node_title_rect.min.y + node_title_rect.size().y / 2.0 };
        ui.painter().text(title_pos, egui::Align2::CENTER_CENTER, self.title.clone(), egui::FontId::default(), egui::Color32::WHITE);

        // Setup UI interactable elements
        let _ = ui.allocate_ui_at_rect(self.node_rect, |ui|
        {
            // Title response
            let title_response = ui.allocate_response(node_title_rect.size(), egui::Sense::click());

            if title_response.clicked()
            {
                self.is_moving = !self.is_moving;

                if self.is_moving == true
                {
                    self.grab_vector = egui::Vec2 
                                            {
                                                x: self.node_rect.min.x - mouse_pos.x, 
                                                y: self.node_rect.min.y - mouse_pos.y
                                            };
                }
            }

            self.node_title_color = egui::Color32::GRAY;
            if title_response.hovered()
            {
                self.node_title_color = egui::Color32::from_rgb(83, 83, 83);
            }

            // Perform specific changes depending on the node type
            if ui.button("test").clicked()
            {
                println!("Was clicked");
            }
        
            if ui.button("find me").clicked()
            {
                println!("Was clicked");
            }
        
            if ui.button("I am here too").clicked()
            {
                println!("Was clicked");
            }
        
            //let quick_button = ui.add_sized([50., 50.], egui::Button::new("I am big"));
        
            if ui.button("I am here too").clicked()
            {
                println!("Was clicked");
            }
            if ui.button("I am here too").clicked()
            {
                println!("Was clicked");
            }
            if ui.button("I am here too").clicked()
            {
                println!("Was clicked");
            }        
        });
    }
}

