pub mod poly
{

pub struct Polygon
{
    name: String,
    sides: u32,
    visible: bool,
}

impl Polygon
{
    pub fn new(name: String) -> Self
    {
        Self{name: name, sides: 3, visible: true}
    }

    pub fn name(self: &Self) -> &String
    {
        &self.name
    }

    pub fn sides(self: &Self) -> u32
    {
        self.sides
    }

    pub fn shape(self: &Self) -> String
    {
        if self.sides == 3
        {
            String::from("Triangle")
        }
        else if self.sides == 4
        {
            String::from("Square")
        }
        else if self.sides == 5
        {
            String::from("Pentagon")
        }
        else
        {
            String::from("Polygon")
        }
    }

    pub fn increment_sides(self: &mut Self)
    {
        self.sides += 1;
    }
}

}