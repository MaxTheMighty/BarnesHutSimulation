use cgmath::num_traits::{Saturating, SaturatingAdd};
use hsv::hsv_to_rgb;

pub struct Canvas{
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<(u8,u8,u8,u8)>,
    pub buffer_default_color: (u8,u8,u8,u8),
    pub huemap: Vec<(f64,f64,f64)>,
    pub length: u32,

}

impl Canvas{
    pub fn new(width: u32, height: u32, buffer_default_color: (u8, u8, u8, u8)) -> Self {
        Self {
            width,
            height,
            length: width * height,
            buffer_default_color,
            buffer: vec![buffer_default_color; ((width * height) + width) as usize],
            huemap: vec![(0.0,0.0,0.0); ((width * height) + width) as usize],
        }
    }



    /// Sets the color at the given x and y position
    ///
    /// This function does not do any checking for bounds, to check for bounds, use `set_color_safe`


    pub fn set_color(&mut self, x_pos: i32, y_pos: i32, color: &(u8,u8,u8,u8)){
        let index: usize = self.get_index(x_pos,y_pos);
        self.buffer[index] = color.clone();
    }

    /// Sets the color at the given x and y position
    ///
    /// This function checks for bounds, and does not set the color if it is out of bounds
    pub fn set_color_safe(&mut self, x_pos: i32, y_pos: i32, color: &(u8,u8,u8,u8)){
        if(!self.pos_valid(x_pos,y_pos)){
            return;
        }
        let index: usize = self.get_index(x_pos,y_pos);
        self.buffer[index] = color.clone();
    }
    //
    // /// Returns a reference to the color at the given index
    //
    // pub fn get_color(&self, x_pos: i32, y_pos: i32) -> &(u8,u8,u8,u8){
    //     let index: usize = self.get_index(x_pos,y_pos);
    //     return &self.buffer[index]; //will this work?
    // }
    //
    //
    // /// Returns a mutable reference to the color at the given index
    // pub fn get_color_mut(&mut self, x_pos: i32, y_pos: i32) -> &mut (u8,u8,u8,u8){
    //     let index: usize = self.get_index(x_pos,y_pos);
    //     return &mut self.buffer[index]; //will this work?
    // }

    /// Returns a reference to the hue from the huemap at a given index
    pub fn get_hue(&self, x_pos: i32, y_pos: i32) -> &(f64,f64,f64){
        let index: usize = self.get_index(x_pos,y_pos);
        return &self.huemap[index];
    }

    /// Returns a mutable reference to the hue from the huemap at a given index
    pub fn get_hue_mut(&mut self, x_pos: i32, y_pos: i32) -> &mut (f64,f64,f64){
        let index: usize = self.get_index(x_pos,y_pos);
        return &mut self.huemap[index];
    }

    /// Sets the hue at the given x and y position
    ///
    /// This function does not do any checking for bounds, to check for bounds, use `set_hue_safe`
    pub fn set_hue(&mut self, x_pos: i32, y_pos: i32, hsv: &(f64,f64,f64)){
        let index: usize = self.get_index(x_pos,y_pos);
        self.huemap[index] = hsv.clone();
    }

    //TODO
    pub fn set_hue_safe(&mut self, x_pos: i32, y_pos: i32, hsv: &(f64,f64,f64)){
        let index: usize = self.get_index(x_pos,y_pos);
        if(!self.pos_valid(x_pos,y_pos)){
            return;
        }

        self.huemap[index] = hsv.clone();
    }

    /// Copy the values from the huemap to the given buffer
    pub fn copy_huemap_to_buffer(&mut self, buffer: &mut [u8]){
        for (huemap_pixel, canvas_pixel) in self.huemap.iter().zip(buffer.chunks_mut(4)){
            let (h,s,v) = *huemap_pixel;
            let rgb_value = hsv_to_rgb(h,s,v);
            canvas_pixel[0] = rgb_value.0;
            canvas_pixel[1] = rgb_value.1;
            canvas_pixel[2] = rgb_value.2;
            canvas_pixel[3] = 255;
        }
    }

    /*
    fn rainbow_color(t: f32) -> [u8; 3] {
    let r = (6.0 * (1.0 - t)).min(1.0).max(0.0);
    let g = (6.0 * (t - 1.0/6.0).abs() - 1.0).min(1.0).max(0.0);
    let b = (6.0 * (t - 2.0/3.0).abs() - 1.0).min(1.0).max(0.0);

    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8
    ]
}
     */
    /// Increments the huemap at a given point, clamping it to 0-360
    pub fn increment_huemap(&mut self, x_pos: i32, y_pos: i32, start_hsv: (f64,f64,f64), increment: f64){
        if(!self.pos_valid(x_pos,y_pos)){
            return;
        }

        let index: usize = self.get_index(x_pos,y_pos);

        if self.huemap[index].0 == 0.0 {
            self.huemap[index] = start_hsv;
        }
        self.huemap[index].0 += increment;

        self.huemap[index].0.clamp(0.0,360.0);

    }


    ///Draws a square using the top left x and y position of the square, and the squares width, IN HSV
    ///Unsafe

    pub fn draw_square_rgb(&mut self, top_left_x: i32, top_left_y: i32, width: i32, height: i32, rgb_color: &(u8, u8, u8, u8)){
        let top_right_x: i32 = top_left_x.saturating_add(width);
        // let bottom_left_y: usize = top_left_y+width-1;
        let bottom_left_y: i32 = top_left_y.saturating_add(height);
        //Top and bottom horizontal line
        for x in top_left_x..=top_right_x{
            self.set_color(x, top_left_y, rgb_color);
            self.set_color(x, top_left_y.saturating_add(height).saturating_sub(0), rgb_color);
        }

        //Left and right vertical line
        for y in top_left_y..=bottom_left_y{
            self.set_color(top_left_x, y, rgb_color);
            self.set_color(top_left_x.saturating_add(width).saturating_sub(0), y, rgb_color);
        }
    }

    ///Draws a square using the top left x and y position of the square, and the squares width and height
    ///Safe

    pub fn draw_square_safe(&mut self, top_left_x: i32, top_left_y: i32, width: i32, height: i32, color: &(f64,f64,f64)){
        let top_right_x: i32 = top_left_x.saturating_add(width);
        // let bottom_left_y: usize = top_left_y+width-1;
        let bottom_left_y: i32 = top_left_y.saturating_add(height);
        //Top and bottom horizontal line
        for x in top_left_x..=top_right_x{
            // self.set_color_safe(x,top_left_y,color);
            // self.set_color_safe(x,top_left_y.saturating_add(height),color);
            self.set_hue_safe(x,top_left_y,color);
            self.set_hue_safe(x,top_left_y.saturating_add(height),color);
        }

        //Left and right vertical line
        for y in top_left_y..=bottom_left_y{
            self.set_hue_safe(top_left_x,y,color);
            self.set_hue_safe(top_left_x.saturating_add(width),y,color);
        }
    }


    /// Checks if a color at a given x and y equals a given color
    pub fn is_color(&self, x_pos: i32, y_pos: i32, color: &(u8,u8,u8,u8)) -> bool{
        let index = self.get_index(x_pos,y_pos);
        return self.buffer[index] == *color;
    }


    /// Converts an `x` and `y` position to an index value for the buffer
    /// Does not do any index boundary checking
    pub fn get_index(&self, x_pos: i32, y_pos: i32) -> usize{
        return ((self.width as i32 * y_pos) + x_pos) as usize;
    }

    pub fn pos_valid(&self, x_pos: i32, y_pos: i32) -> bool {
        return x_pos >= 0 && x_pos < self.width as i32 && y_pos >= 0 && y_pos < self.height as i32;
    }

    pub fn is_default(&self, x_pos: i32, y_pos: i32) -> bool{
        return self.is_color(x_pos,y_pos,&self.buffer_default_color);
    }

    pub fn clear(&mut self){
        self.buffer.fill((0,0,0,0));
        self.huemap.fill((0.0,0.0,0.0));
    }
}



#[cfg(test)]
mod tests{
    use crate::canvas::Canvas;

    #[test]
    fn test_indexing(){
        let canvas: Canvas = Canvas::new(1000,1000, (0,0,0,0));
        assert_eq!(canvas.get_index(0,0),0);
        assert_eq!(canvas.get_index(1,0),1);
        assert_eq!(canvas.get_index(0,1), canvas.width as usize);
        assert_eq!(canvas.get_index(1,1), (canvas.height + 1) as usize);
        assert_eq!(canvas.get_index(1000,1000), (canvas.height*canvas.width + canvas.width) as usize);
    }

    #[test]
    fn test_canvas_size(){
        let canvas: Canvas = Canvas::new(5000,5000, (0,0,0,0));
        assert_eq!(canvas.buffer.len(), 5000*5000 + 5000);
    }

    #[test]
    fn test_is_color(){
        let mut canvas: Canvas = Canvas::new(1000,1000, (0,0,0,0));
        canvas.set_color(0,0,&(1,2,3,4));
        assert_eq!(canvas.is_color(0,0,&(1,2,3,4)),true);
        assert_eq!(canvas.is_color(0,0,&(1,2,3,3)),false);
    }


}

