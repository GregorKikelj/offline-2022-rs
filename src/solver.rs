use crate::importer::TestCase;
use std::collections::VecDeque;
use rand::Rng;

///Id, n, c(number of colors), col(node colors), edge(edges), 
pub struct ReducedTC{
  pub id: i32,
  pub n: usize,
  pub col: i32,
  pub c:Vec<i8>,
  pub edge: Vec<Vec<i32>>, 
}

impl ReducedTC{
  pub fn convert_tc(tc:TestCase) -> Self{
    let mut done : Vec<Vec<bool>> = vec![vec![false; tc.n]; tc.n];
    let mut id : Vec<Vec<i32>> = vec![vec![-1;tc.n];tc.n];
    let mut n=0;
    for x in 0..tc.n {
      for y in 0..tc.n {
        if !done[x][y] {
          n+=1;
          let mut bfs = VecDeque::with_capacity(100);
          bfs.push_front((x as i32, y as i32));
          done[x][y] = true;
          id[x][y] = n-1;
          while !bfs.is_empty(){
            let (x, y) = bfs.pop_back().expect("bfs is not Empty");
            let xi = x as i32;
            let yi = y as i32;
            let col = tc.data[x as usize][y as usize];
            for (x2, y2) in [(xi+1, yi), (xi, yi+1), (xi-1, yi), (xi, yi-1)]{
              if x2<0 || x2>=tc.n as i32 || y2<0 || y2>=tc.n as i32 {
                continue;
              }
              if tc.data[x2 as usize][y2 as usize]!=col || done[x2 as usize][y2 as usize]{
                continue;
              }
              done[x2 as usize][y2 as usize]=true;
              id[x2 as usize][y2 as usize]=n-1;
              bfs.push_front((x2, y2));
            }
          }
        }
      }
    }

    let n = n as usize;// Now it's not mut anymore
    let mut edge : Vec<Vec<i32>> = vec![vec![]; n];
    let mut c : Vec<i8> = vec![-1;n];
    for x in 0..tc.n {
      for y in 0..tc.n {
        let xi = x as i32;
        let yi = y as i32;
        let col = tc.data[x][y];
        for (x2, y2) in [(xi+1, yi), (xi, yi+1), (xi-1, yi), (xi, yi-1)]{
          if x2<0 || x2>=tc.n as i32 || y2<0 || y2>=tc.n as i32 {
            continue;
          }
          let id1 = id[x][y];
          c[id1 as usize]=col as i8;
          if tc.data[x2 as usize][y2 as usize]!=col {
            //Add connection between id and id2
            let id2 = id[x2 as usize][y2 as usize];
            if !edge[id1 as usize].contains(&(id2 as i32)) {
              edge[id1 as usize].push(id2 as i32);
            }
          }
        }
      }
    }
    //Sort the weights, probably not needed, but it doesn't hurt :)
    //And I don't really care about performance here, it's just a converter between one way of presenting data into another
    for i in 0..n {
      edge[i].sort_unstable();
    }

    return ReducedTC{
      id:tc.id,
      n:n as usize,
      col:tc.col,
      c:c,
      edge:edge,
    };
  }
}


fn add_color(tc:&ReducedTC, done:&mut Vec<bool>, col:i8) -> i32{
  let mut cnt=0;
  for i in 0..tc.n {
    if done[i] {
      for j in &tc.edge[i] {
        if tc.c[*j as usize]==col && !done[*j as usize] {
          done[*j as usize] = true;
          cnt+=1;
        }
      }
    }
  }
  return cnt;
}
pub fn random_move_solver(tc:ReducedTC) -> Vec<i8>{
  let mut ans = vec![];
  let mut done = vec![false; tc.n];
  done[0]=true;
  let mut cnt_done : i32 = 1;
  while cnt_done < tc.n as i32 {
    let color:i8 = (rand::thread_rng().gen_range(0..tc.col) + 1) as i8;
    let d = add_color(&tc, &mut done, color);
    cnt_done += d;
    ans.push(color);
  }
  return ans;
}

