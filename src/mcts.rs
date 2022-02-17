use crate::solver::{ReducedTC, add_color, random_move_solver_heuristics};
use once_cell::sync::OnceCell;
// use rand::Rng;
// use std::{thread, time::Duration};


static TC: OnceCell<ReducedTC> = OnceCell::new();

struct MctsNode {
  plays: i32,
  sum_score: i32,
  done: bool,

  colored: Vec<bool>,
  ch: Vec<ChOpts>,
}

#[derive(Debug)]
enum ChOpts{
  Unexplored,
  Useless,
  Node(MctsNode),
}
impl std::fmt::Debug for MctsNode {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
    return write!(f,"\n{}", self.to_string()); 
  }
}
impl MctsNode{
  fn new(n:usize) -> Self{
    return MctsNode{
      plays:0,
      sum_score:0,
      done:false,

      colored:vec![false; n],
      ch: {
        let len = TC.get().expect("").col;
        let mut ch = Vec::with_capacity(len as usize);
        for _ in 0..len {
          ch.push(ChOpts::Unexplored);
        }
        ch
      }
    };
  }
  fn from_done(dcolored:Vec<bool>) -> Self{
    return MctsNode{
      plays:0,
      sum_score:0,
      done:{
        let mut done = true;
        for c in &dcolored {
          if !c {
            done = false;
          }
        }
        done
      },

      colored:dcolored,
      ch:{
        let len = TC.get().expect("").col;
        let mut ch = Vec::with_capacity(len as usize);
        for _ in 0..len {
          ch.push(ChOpts::Unexplored);
        }
        ch
      }
    };
  }

  fn add_score(&mut self, score:i32) {
    self.sum_score+=score;
    self.plays+=1;
  }
  fn playout(&mut self) -> i32{
    let mut col2 = self.colored.clone();
    let ans = random_move_solver_heuristics(TC.get().expect(""), &mut col2);
    self.add_score(ans.len() as i32);
    return ans.len() as i32;
  }
  fn get_score(&self, par_plays:i32) -> f64 {
    let splays = self.plays as f64;
    let exp_moves = (self.sum_score as f64)/splays;
    return exp_moves - 20.0*((par_plays as f64).ln()/splays).sqrt()
  }
  fn mcts(&mut self) -> i32{
    if self.done {
      return 0;
    }

    //Expansion process
    for i in 0..self.ch.len() {
      match &self.ch[i] {
        ChOpts::Unexplored => {
          let mut colored_ch = self.colored.clone();
          let t = add_color(TC.get().expect(""), &mut colored_ch, (i+1) as i8);
          if t==0 {
            self.ch[i]=ChOpts::Useless;
            continue;
          }

          self.ch[i] = ChOpts::Node(MctsNode::from_done(colored_ch));
          if let ChOpts::Node(ch) = &mut self.ch[i] {
            let score = ch.playout()+1;
            self.add_score(score);
            return score;
          }else{
            panic!("Types don't match up")
          }
        },
        _ => (),
      }
    }

    // Choose best node for mcts
    let mut best_node:Option<&mut MctsNode> = None;
    let mut best_node_score = 100000.0;
    for ch in self.ch.iter_mut() {
      if let ChOpts::Node(ref mut ch_node) = ch {
        let score = ch_node.get_score(self.plays);
        if score < best_node_score{
          best_node = Some(ch_node);
          best_node_score = score;
        }
      }
    }

    if let Some(ref mut node) = best_node {
      let moves = node.mcts()+1;
      self.add_score(moves);
      return moves;
    }else{
      panic!("No nodes can be explored");
    }
  }
  fn to_string(&self) -> String{
    let mut resp = String::from("");
    for i in 0..self.ch.len() {
      let line = match &self.ch[i] {
        ChOpts::Unexplored => String::from("UNEXPLORED"),
        ChOpts::Useless => String::from("--------"),
        ChOpts::Node(mnode) => format!(" {}, {}", mnode.plays, (mnode.sum_score as f64)/(mnode.plays as f64)), 
      };
      resp.push_str(i.to_string().as_str());
      resp.push_str(line.as_str());
      resp.push_str("\n");
    }
    return resp;
  }
}


pub fn solve(g:ReducedTC) -> Vec<i8> {
  let _t = TC.set(g);// Not sure why let is needed, rust is annoying sometimes
  let g = TC.get().expect("");
  let mut ans:Vec<i8> = vec![];
  let mut m_node: &mut MctsNode = &mut MctsNode::new(g.n);
  m_node.colored[0]=true;
  let cycles = 10000;
  while !m_node.done{
    println!("Running {}", ans.len());
    for _ in 0..cycles {
      let _s = m_node.mcts();
    }
    println!("{}", m_node.to_string());

    let mut best_node:i32 = -1;
    let mut best_node_score = 100000.0;
    for i in 0..m_node.ch.len(){
      let ch = &m_node.ch[i];
      if let ChOpts::Node(ch_node) = ch {
        let score = (ch_node.sum_score as f64)/(ch_node.plays as f64);
        if score < best_node_score{
          best_node = i as i32;
          best_node_score = score;
        }
      }
    }
    let t:&mut ChOpts = &mut {m_node}.ch[best_node as usize];

    if let ChOpts::Node(next) = t {
      m_node = next;
      ans.push(g.c[best_node as usize])
    }else{
      panic!("Can't use child for mcts");
    }
    m_node.done=true;
    // thread::sleep(Duration::from_millis(1000));
    // break;
  }
  return ans;
}
