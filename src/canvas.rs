pub struct Canvas{
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<(u8,u8,u8,u8)>,
    pub length: usize
}

impl Canvas{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            length: width * height,
            buffer: vec![(0, 0, 0, 0); (width * height) + width + 1]
        }
    }

    /// Sets the color at the given x and y position

    pub fn set_color(&mut self, x_pos: usize, y_pos: usize, color: &(u8,u8,u8,u8)){
        let index: usize = self.get_index(x_pos,y_pos);
        self.buffer[index] = color.clone();
    }


    /// Returns a reference to the color at the given index

    pub fn get_color(&mut self, x_pos: usize, y_pos: usize) -> &(u8,u8,u8,u8){
        let index: usize = self.get_index(x_pos,y_pos);
        return &self.buffer[index]; //will this work?
    }


    ///Draws a square using the top left x and y position of the square, and the squares width

    pub fn draw_square(&mut self, top_left_x: usize, top_left_y: usize, width: usize, color: &(u8,u8,u8,u8)){
        let top_right_x: usize = top_left_x+width-1;
        let bottom_left_y: usize = top_left_y+width-1;
        //Top and bottom horizontal line
        for x in top_left_x..=top_right_x{
            self.set_color(x,top_left_y,color);
            self.set_color(x,top_left_y+width-1,color);
        }

        //Left and right vertical line
        for y in top_left_y..=bottom_left_y{
            self.set_color(top_left_x,y,color);
            self.set_color(top_left_x+width-1,y,color);
        }


    }



    /// Converts an `x` and `y` position to an index value for the buffer
    pub fn get_index(&self, x_pos: usize, y_pos: usize) -> usize{
        // x_pos = 0 and y_pos = 0 should equal 0

        return (self.width * y_pos) + x_pos;
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
        let canvas: Canvas = Canvas::new(1000,1000);
        assert_eq!(canvas.get_index(0,0),0);
        assert_eq!(canvas.get_index(1,0),1);
        assert_eq!(canvas.get_index(0,1),canvas.height);
        assert_eq!(canvas.get_index(1,1), canvas.height+1);
        assert_eq!(canvas.get_index(1000,1000), canvas.height*canvas.width + 1000);
    }

    #[test]
    fn test_canvas_size(){
        let canvas: Canvas = Canvas::new(5000,5000);
        assert_eq!(canvas.buffer.len(), 5000*5000 + 5000);
    }


}

