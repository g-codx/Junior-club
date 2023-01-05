

pub struct Path {
    pub prefix: String
}

impl Path {

    pub fn define(&self, following_path: String) -> String {
        let path: String = self.prefix.to_owned() + &following_path;
        path
    }

}