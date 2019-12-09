use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    g_cost: i32,    //* Distance to Start Point
    h_cost: i32,    //* Distance to End Point
    f_cost: i32,    //* 'Value' of the Node
    x: i32,   //x Position 
    y: i32,   //y Position
}
impl Node {
    pub fn new(new_x: i32, new_y: i32, new_g_cost: i32) -> Node {
        let ret_node = Node {
            g_cost: new_g_cost,
            h_cost: 0,
            f_cost: i32::max_value(),
            x: new_x as i32,
            y: new_y as i32,
        };
        
        ret_node
    }

    fn update_f_cost(&mut self, end_node: &Node) {
        let dis_x = (self.x - end_node.x).abs();
        let dis_y = (self.y - end_node.y).abs();
        let smallest = if dis_x < dis_y { dis_x } else { dis_y };

        let new_h_cost =  14 * smallest + 10 * (dis_x - smallest + dis_y - smallest);

        if self.f_cost > new_h_cost + self.g_cost {
            self.f_cost = new_h_cost + self.g_cost;
            self.h_cost = new_h_cost;
        }
    }

    fn get_new_h_cost(end_node: &Node, new_node: &Node) -> i32 {
        let dis_x = new_node.x - end_node.x;
        let dis_y = new_node.y - end_node.y;
        let smallest = if dis_x < dis_y { dis_x.abs() } else { dis_y.abs() };
        
        14 * smallest + 10 * (dis_x.abs() - smallest + dis_y.abs() - smallest)
    }

    fn get_lowest(list: &HashMap<(i32, i32), Node>) -> (i32, i32) {
        let (old_key, old_value) = list.iter().nth(0).expect("get_lowest received an empty List");

        let mut lowest_node: &Node  = old_value;
        let mut lowest_key: &(i32, i32) = old_key;

        for (key, value) in list.iter() {
            if value.f_cost < lowest_node.f_cost {
                lowest_node = value;
                lowest_key = key;
                continue;
            }
            if value.f_cost == lowest_node.f_cost {
                if value.h_cost < lowest_node.h_cost {
                    lowest_node = value;
                    lowest_key = key;
                }
            }
        }

        lowest_key.clone()
    }

    pub fn loop_neighbours(start_node: &Node, target_node: &Node, map: &mut[&mut[bool]], open_list: &mut HashMap<(i32, i32), Node>) -> HashMap<(i32, i32), Node> {
        const OFFSET: [(i32, i32); 8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1, 0), (-1,1), (0,1), (1,1)];
        let y_size = map.len() - 1;
        let x_size = map[0].len() - 1;

        if x_size == 0  || y_size == 0 {
            panic!("open_list is empty, or to small (in loop_neighbours)");
        }

        let mut closed_list = HashMap::new();

        let mut tmp_node = start_node.clone();
        tmp_node.update_f_cost(&target_node);
        closed_list.insert((start_node.x, start_node.y), tmp_node);
        
        loop { 
            let current_node = open_list.remove( &Node::get_lowest(&open_list) ).expect("could not remove lowest value");
            closed_list.insert((current_node.x, current_node.y), current_node.clone());

            //check for end condition:
            if current_node.x == target_node.x && current_node.y == target_node.y {
                println!("Shoutest path has been found!");
                break;
            }

            //loop threw neighbours
            for &(x_off, y_off) in OFFSET.iter() {
                let mut neighbour: Node;

                //set g_cost of the neighboour
                if x_off == 0 || y_off == 0 {
                    neighbour = Node::new(current_node.x + x_off, current_node.y + y_off, current_node.g_cost + 10);
                }
                else {
                    neighbour = Node::new(current_node.x + x_off, current_node.y + y_off, current_node.g_cost + 14);
                }
                
                //if neighbour is not a valid tile
                if neighbour.x < 0 || neighbour.x > x_size as i32 {
                    continue;
                }
                if neighbour.y < 0 || neighbour.y > y_size as i32 {
                    continue;
                }
                if map[ neighbour.y as usize ][ neighbour.x as usize ] == true {
                    continue;
                }
                if match closed_list.get(&(neighbour.x, neighbour.y)) {Some(_x) => true, None => false,} {
                    continue;
                }
                
                //if new path to neighbour is shorter || neighbour is not in open_list
                if neighbour.f_cost > neighbour.g_cost + Node::get_new_h_cost(&target_node, &neighbour) || match open_list.get(&(neighbour.x, neighbour.y)) {Some(_x) => false, None => true,} {
                    neighbour.update_f_cost(&target_node);
                    
                    /* neighbour is not in open_list */
                    if match open_list.get(&(neighbour.x, neighbour.y)) {Some(_x) => false, None => true,} {
                        open_list.insert((neighbour.x, neighbour.y), neighbour);
                    }
                }
            }

            //check if there is a possible move next turn
            if open_list.is_empty() {
                panic!("No Path !");
            }
        }
        closed_list
    }

    pub fn init_neighbours(start_node: &Node, target_node: &Node, map: &mut[&mut[bool]]) ->HashMap<(i32, i32), Node> {
        const OFFSET: [(i32, i32); 8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1, 0), (-1,1), (0,1), (1,1)];
        let y_size = map.len() - 1;
        let x_size = map[0].len() - 1;

        if x_size == 0  || y_size == 0 {
            panic!("open_list is empty, or to small (in init_neighbours)");
        }

        let mut open_list = HashMap::new();

        //insert starting values into open list:
        for &(x_off, y_off) in OFFSET.iter() {
            let mut neighbour: Node;
            if x_off == 0 || y_off == 0 {
                neighbour = Node::new(start_node.x + x_off, start_node.y + y_off, 10);
            }
            else {
                neighbour = Node::new(start_node.x + x_off, start_node.y + y_off, 14);
            }

            if neighbour.x < 0 || neighbour.x > x_size as i32 {
                continue;
            }
            if neighbour.y < 0 || neighbour.y > y_size as i32 {
                continue;
            }
            if map[ neighbour.y as usize ][ neighbour.x as usize ] == true {
                continue;
            }

            neighbour.update_f_cost(&target_node);

            //add new node to open list
            open_list.insert((start_node.x + x_off, start_node.y + y_off), neighbour);
        }
        open_list
    }

    pub fn get_path(start_node: &Node, target_node: &Node, closed_list: &HashMap<(i32, i32), Node>) -> Vec<(i32,i32)> {
        const OFFSET: [(i32, i32); 8] = [(-1,-1), (0,-1), (1,-1), (-1,0), (1, 0), (-1,1), (0,1), (1,1)];
        let mut path_list: Vec<(i32,i32)> = Vec::new();
        let mut current_pos = (target_node.x, target_node.y);
        let mut lowest: &Node = closed_list.get(&current_pos).expect("closed List did not contain target Node!");
        
        path_list.push(current_pos);

        loop {
            for &(x_off, y_off) in OFFSET.iter() {
                let new_node: &Node;
                new_node = match closed_list.get(&(current_pos.0 + x_off, current_pos.1 + y_off)){
                    Some(v) => v,
                    None => continue,
                };

                //check if new node is lower than last
                if new_node.f_cost < lowest.f_cost {
                    lowest = new_node;
                    continue;
                }
                if new_node.f_cost == lowest.f_cost {
                    if new_node.g_cost < lowest.g_cost {
                        lowest = new_node;
                    }
                }
            }

            current_pos = (lowest.x, lowest.y);
            path_list.push(current_pos);

            if current_pos == (start_node.x, start_node.y) {
                //path has been complete!
                break;
            }
        }
        path_list.reverse();
        path_list
    }   
}