use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::ffi::{CString, CStr};
use linkroot;
use std::os::raw::{c_char, c_void};
use std::cell::RefCell;
use {getshfunc, doshfunc, newlinklist, insertlinknode, linknode, LinkList, getaparam};
use std::mem;
use std::ops::DerefMut;
use std::ptr::null_mut;

/*struct IntHasher;

impl Hasher for IntHasher {

    fn write(&mut self, bytes: &[u8]) {
		assert!(bytes.len(), mem::size_of::<usize>());

    }

    fn finish(&self) -> u64 {
        // Your hashing algorithm goes here!
        unimplemented!()
    }
}*/

pub fn brackets_paint(bracket_color_size: usize, buf: &str, cursor: usize, widget: &str) {
    let mut level: usize = 0;

    // We keep the full usize range of level by tracking when the level goes negative separately
    // Due to the way the highlighting logic works, a bracket match can never occur on a negative
    // level. This allows us to skip tracking information for negative levels, which would make it
    // difficult to use a Vec
    let mut level_neg: usize = 0;

    let mut cursor_level = false;
    let mut level_pos: Vec<(usize, usize)> = Vec::new();
    let mut last_of_level: Vec<usize> = Vec::new();
    //let mut matching: HashMap<usize, usize> = HashMap::new();

    let chars: Vec<(char, RefCell<Option<usize>>)> = buf.chars().map(|c| (c, RefCell::new(None))).collect();

    let mut it = chars.iter().enumerate();
    while let Some((i, &(ref chr, ref match_pos))) = it.next() {
        match *chr {
            '(' | '[' | '{' => {
                if level_neg == 0 {
                    level += 1;
                    if last_of_level.get(level - 1).is_some() {
                        *last_of_level.get_mut(level - 1).unwrap() = i;
                    } else {
                        last_of_level.push(i);
                    }
                } else {
                    level_neg -= 1;
                }

                level_pos.push((i, level));
            },
            ')' | ']' | '}' => {
                level_pos.push((i, level));

                if level == 0 {
                    level_neg += 1;
                    continue;
                }


                let matching_pos: Option<&usize> = last_of_level.get(level - 1);
                level -= 1;

                if brackets_match(matching_pos.and_then(|p| chars.get(*p).map(|s| s.0)).unwrap_or(' '), chars[i].0) {
                    let matching_pos = *matching_pos.unwrap();
                    //
                    *match_pos.borrow_mut() = Some(matching_pos);
                    *(chars.get(matching_pos).unwrap().1.borrow_mut()) = Some(i);

                    //matching.insert(matching_pos, i);
                    //matching.insert(i, matching_pos);
                }
            },
            '\"' | '\'' => {
                while let Some((_, &(ref character, _))) = it.next() {
                    if *character != *chr {
                        continue;
                    }
                    break;
                }
            },
            _ => continue
        }
    }


    for &(pos, level) in level_pos.iter() {
        if cursor == pos {
            cursor_level = true;
        }

        if chars[pos].1.borrow().is_some() {
            if bracket_color_size != 0 {
                do_highlight(pos, pos + 1, &format!("bracket-level-{}", (level - 1) % bracket_color_size + 1));
            }
        } else {
            do_highlight(pos, pos + 1, &"bracket-error");
        }
    }

    if widget != "zle-line-finish" {
        let pos = cursor; // cursor is already zero-based
        if cursor_level {
            let other_pos = chars[pos].1.borrow();
            if let Some(real_pos) = *other_pos {
                do_highlight(real_pos, real_pos + 1, "cursor-matchingbracket");
            }
        }
    }

}

fn brackets_match(first: char, second: char) -> bool {
    match first {
        '(' => second == ')',
        '[' => second == ']',
        '{' => second == '}',
        _ => false,
    }
}

#[cfg(not(test))]
fn do_highlight(start: usize, end: usize, style: &str) {
    unsafe {
        let func_name = str_to_ptr("_zsh_highlight_add_highlight");
        let func = getshfunc(func_name as *mut c_char);

        let list = newlinklist();
        insertlinknode(list, latest_node(list), func_name);
        insertlinknode(list, latest_node(list), str_to_ptr(&start.to_string()));
        insertlinknode(list, latest_node(list), str_to_ptr(&end.to_string()));
        insertlinknode(list, latest_node(list), str_to_ptr(style));

        doshfunc(func, list as *mut linkroot, 1);

        /*let mut param_ptr = getaparam(str_to_ptr("region_highlight") as *mut c_char);
        let mut highlights: Vec<String> = Vec::new();
        if param_ptr != null_mut() {
            while *param_ptr != null_mut() {
                highlights.push(CStr::from_ptr(*param_ptr as *const c_char).to_owned().into_string().unwrap());
                param_ptr = param_ptr.offset(1)
            }
        }*/
        //println!("Result: {:?}", highlights);
    }
}

fn latest_node(list: LinkList) -> *mut linknode {
    unsafe { (*list).list }.last as *const linknode as *mut linknode
}

#[cfg(test)]
fn do_highlight(start: usize, end: usize, style: &str) {
}


fn str_to_ptr(s: &str) -> *mut c_void {
    CString::new(s.to_string()).unwrap().into_raw() as *mut c_void
}
