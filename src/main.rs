#![feature(slice_concat_ext)]
use std::slice::SliceConcatExt;

#[derive(Debug)]
pub enum AstType {
  Root,
  Start,
  End,
  Normal,
  Delimiter
}

#[derive(Debug)]
pub struct PartArena {
  pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
  id: usize,
  start: usize,
  end: usize,
  kind: AstType,
  value: char,
  children: Vec<usize>,
  parent: Option<usize>,
}

impl Part {
  fn new(kind: AstType, imput: char, start: usize) -> Part {
    Part {
      id: 0,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: None,
    }
  }

  fn add_child(&mut self, id: usize, kind: AstType, imput: char, start: usize) -> Part {
    let p = Part {
      id: id,
      start: start,
      end: 0,
      kind: kind,
      value: imput,
      children: Vec::new(),
      parent: Some(self.id),
    };
    self.children.push(p.id);
    p
  }
}

#[derive(Debug)]
pub enum WalkingType {
  Function,
  Normal,
}

#[derive(Debug)]
struct Walker<'a> {
  input: &'a str,
  next: usize,
  part: usize,
  part_arena: PartArena,
  current_type: WalkingType,
}

impl <'a>Walker<'a> {
  fn new(input: &str) -> Walker {
    let mut pa = PartArena{ parts: Vec::new() };
    pa.parts.push(Part::new(
      AstType::Root, ' ', 0
    ));
    
    Walker {
      input: input,
      next: 0,
      part: 0,
      part_arena: pa,
      current_type: WalkingType::Function,
    }
  }

  pub fn walk(&mut self) {
    let mut chars = self.input.chars();
    let mut index: usize = 0;
    let mut arena_id: usize = 0;
    let mut new_part: Part;
    
    while let Some(cha) = chars.next() {
      {
        let mut part = &mut self.part_arena.parts.get_mut(arena_id).unwrap();
        new_part = match cha {
          '{' => {
            let child_part = part.add_child(index + 1, AstType::Start, cha, index);
            arena_id = child_part.id;
            child_part
          },
          '}' => {
            arena_id = part.parent.unwrap();
            part.add_child(index + 1, AstType::Start, cha, index)
          },
          ' ' => {
            part.add_child(index + 1, AstType::Delimiter, cha, index)
          },
          _ => {
            part.add_child(index + 1, AstType::Normal, cha, index)
          }
        };
      }
      self.part_arena.parts.push(new_part);
      index += 1;
    }
  }
}

#[derive(Debug)]
struct Tokens<'a> {
  strs: Vec<&'a str>,
  chars: Vec<char>
}

impl <'a>Tokens<'a> {
  pub fn new() -> Tokens<'a> {
    Tokens {
      strs: Vec::new(),
      chars: Vec::new(),
    }
  }

  pub fn add_str(&mut self, part: &Part) {
    match part.kind {
      AstType::Normal => {
        self.chars.push(part.value);
      },
      AstType::Delimiter => {
        // // let hoge: String = self.chars.into_iter().collect::<String>();
        // let hoge = *self.chars.into_iter().collect::<&str>();
        // // let hoge = self.chars.join(" ");
        // // let hoge = ["a", "b"].concat();
        // println!("{:?}", hoge);
      }
      _ => {}
    };
  }
}

fn main() {
  let mut walker = Walker::new("{a {b  c} } ");
  walker.walk();

  let mut tokens = Tokens::new();

  for part in walker.part_arena.parts.iter() {
    &tokens.add_str(part);
  }
  println!("{:?}", tokens);


}
