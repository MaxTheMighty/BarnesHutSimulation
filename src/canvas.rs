use cgmath::num_traits::{Saturating, SaturatingAdd};

pub struct Canvas{
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<(u8,u8,u8,u8)>,
    pub length: u32,
    pub default: (u8,u8,u8,u8)
}

impl Canvas{
    pub fn new(width: u32, height: u32, default: (u8, u8, u8, u8)) -> Self {
        Self {
            width,
            height,
            length: width * height,
            default,
            buffer: vec![default; ((width * height) + width) as usize],
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

    /// Returns a reference to the color at the given index

    pub fn get_color(&self, x_pos: i32, y_pos: i32) -> &(u8,u8,u8,u8){
        let index: usize = self.get_index(x_pos,y_pos);
        return &self.buffer[index]; //will this work?
    }


    /// Returns a mutable reference to the color at the given index
    pub fn get_color_mut(&mut self, x_pos: i32, y_pos: i32) -> &mut (u8,u8,u8,u8){
        let index: usize = self.get_index(x_pos,y_pos);
        return &mut self.buffer[index]; //will this work?
    }


    ///Draws a square using the top left x and y position of the square, and the squares width
    ///Unsafe

    pub fn draw_square(&mut self, top_left_x: i32, top_left_y: i32, width: i32, height: i32, color: &(u8,u8,u8,u8)){
        let top_right_x: i32 = top_left_x.saturating_add(width).saturating_sub(0);
        // let bottom_left_y: usize = top_left_y+width-1;
        let bottom_left_y: i32 = top_left_y.saturating_add(height).saturating_sub(0);
        //Top and bottom horizontal line
        for x in top_left_x..=top_right_x{
            self.set_color(x,top_left_y,color);
            self.set_color(x,top_left_y.saturating_add(height).saturating_sub(0),color);
        }

        //Left and right vertical line
        for y in top_left_y..=bottom_left_y{
            self.set_color(top_left_x,y,color);
            self.set_color(top_left_x.saturating_add(width).saturating_sub(0),y,color);
        }
    }

    ///Draws a square using the top left x and y position of the square, and the squares width and height
    ///Safe

    pub fn draw_square_safe(&mut self, top_left_x: i32, top_left_y: i32, width: i32, height: i32, color: &(u8,u8,u8,u8)){
        let top_right_x: i32 = top_left_x.saturating_add(width).saturating_sub(0);
        // let bottom_left_y: usize = top_left_y+width-1;
        let bottom_left_y: i32 = top_left_y.saturating_add(height).saturating_sub(0);
        //Top and bottom horizontal line
        for x in top_left_x..=top_right_x{
            self.set_color_safe(x,top_left_y,color);
            self.set_color_safe(x,top_left_y.saturating_add(height).saturating_sub(0),color);
        }

        //Left and right vertical line
        for y in top_left_y..=bottom_left_y{
            self.set_color_safe(top_left_x,y,color);
            self.set_color_safe(top_left_x.saturating_add(width).saturating_sub(0),y,color);
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
        return self.is_color(x_pos,y_pos,&self.default);
    }

    pub fn clear(&mut self){
        self.buffer.fill((0,0,0,0));
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

