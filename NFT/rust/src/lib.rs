// this is poorly written rust code (I am a C++ developer), so please bear with me!
// if you have any questions, contact me at: six.contrast@gmail.com

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
extern crate nalgebra_glm as glm;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// code to implement console_log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}






pub struct SimpleStrip {
    start_position: glm::Vec3,
    end_position: glm::Vec3,
    start_color: glm::Vec3,
    end_color: glm::Vec3,
    normal: glm::Vec3,
    thickness: f32,
    division: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexStrip {
    start_position: [f32; 3],
    start_normal: [f32; 3],
    start_direction: [f32; 3],
    start_color: [f32; 3],

    end_position: [f32; 3],
    end_normal: [f32; 3],
    end_direction: [f32; 3],
    end_color: [f32; 3],

    start_thickness: f32,
    end_thickness: f32,
    division: i32,
}


#[derive(Debug)]
pub struct Vertex {
    position: glm::Vec3,
    normal: glm::Vec3,
    color: glm::Vec3
}

pub fn points_to_quad(points: &[glm::Vec3], normals: &[glm::Vec3], colors: &[glm::Vec3]) -> Vec<Vertex>
{
    let mut vertices: Vec<Vertex> = Vec::new();
    let indices = [0, 1, 3, 0, 3, 2];
    for i in 0 .. indices.len() {
        let vtx: Vertex = Vertex {
            position: points[indices[i]],
            normal: normals[indices[i]],
            color: colors[indices[i]],
        };
        vertices.push(vtx);
    }
    return vertices;
}

pub fn strip_to_vertices(points: Vec<glm::Vec3>, normals: Vec<glm::Vec3>, colors: Vec<glm::Vec3>) -> Vec<Vertex>
{
    let mut vertices: Vec<Vertex> = Vec::new();

    let quad_count = (points.len() - 2) / 2;
    for i in 0 .. quad_count {
        let mut quad = points_to_quad(&points[2 * i .. 2 * (i + 2)], &normals[2 * i .. 2 * (i + 2)], &colors[2 * i .. 2 * (i + 2)]);
        vertices.append(&mut quad);
    }

    return vertices;
}







pub fn complex_strip(strip: ComplexStrip) -> Vec<Vertex>
{
    let start_position = glm::vec3(strip.start_position[0], strip.start_position[1], strip.start_position[2]);
    let end_position = glm::vec3(strip.end_position[0], strip.end_position[1], strip.end_position[2]);
    let start_direction = glm::vec3(strip.start_direction[0], strip.start_direction[1], strip.start_direction[2]);
    let end_direction = glm::vec3(strip.end_direction[0], strip.end_direction[1], strip.end_direction[2]);
    let start_normal = glm::vec3(strip.start_normal[0], strip.start_normal[1], strip.start_normal[2]);
    let end_normal = glm::vec3(strip.end_normal[0], strip.end_normal[1], strip.end_normal[2]);
    let start_color = glm::vec3(strip.start_color[0], strip.start_color[1], strip.start_color[2]);
    let end_color = glm::vec3(strip.end_color[0], strip.end_color[1], strip.end_color[2]);


    let mut s_dir = start_direction;
    if glm::length(&s_dir) < 0.001 {
        s_dir = glm::normalize(&(&end_position - &start_position));
    }
    let s_tangent = glm::normalize(&glm::cross(&start_normal, &s_dir));
    let s_top = start_position + 0.5 * strip.start_thickness * s_tangent;
    let s_bot = start_position - 0.5 * strip.start_thickness * s_tangent;

    let mut e_dir = end_direction;
    if glm::length(&e_dir) < 0.001 {
        e_dir = glm::normalize(&(&end_position - &start_position));
    }
    let e_tangent = glm::normalize(&glm::cross(&end_normal, &e_dir));
    let e_top = end_position + 0.5 * strip.end_thickness * e_tangent;
    let e_bot = end_position - 0.5 * strip.end_thickness * e_tangent;

    let ddir = s_dir - e_dir;
    let mut pts: Vec<glm::Vec3> = Vec::new();
    let mut normals: Vec<glm::Vec3> = Vec::new();
    let mut colors: Vec<glm::Vec3> = Vec::new();
    for i in 0..(strip.division + 1) {

        let alpha = (i as f32) / (strip.division as f32);
        let mut top = glm::mix(&s_top, &e_top, alpha) + alpha * (1.0 - alpha) * ddir;
        let mut bottom = glm::mix(&s_bot, &e_bot, alpha) + alpha * (1.0 - alpha) * ddir;
        let middle = 0.5 * (top + bottom);
        let tang = glm::normalize(&(&top - &bottom));

        let thickness = (1.0 - alpha) * strip.start_thickness + alpha * strip.end_thickness;
        top = middle + 0.5 * thickness * tang;
        bottom = middle - 0.5 * thickness * tang;
        
        pts.push(top);
        pts.push(bottom);

        let direction = glm::mix(&s_dir, &e_dir, alpha);
        let normal = glm::normalize(&glm::cross(&direction, &(&top - &bottom)));

        normals.push(normal);
        normals.push(normal);

        let color = glm::mix(&start_color, &end_color, alpha);
        colors.push(color);
        colors.push(color);
    }

    let vertices = strip_to_vertices(pts, normals, colors);

    return vertices;
}





pub fn simple_strip(strip: SimpleStrip) -> Vec<Vertex>
{
    let mut pts: Vec<glm::Vec3> = Vec::new();
    let mut normals: Vec<glm::Vec3> = Vec::new();
    let mut colors: Vec<glm::Vec3> = Vec::new();

    let direction = glm::normalize(&(&strip.end_position - &strip.start_position));
    let tangent = glm::normalize(&glm::cross(&strip.normal, &direction));
    let segment = glm::length(&(&strip.end_position - &strip.start_position)) / (strip.division as f32);

    let top: glm::Vec3 = strip.start_position + 0.5 * strip.thickness * tangent;
    let bottom: glm::Vec3 = strip.start_position - 0.5 * strip.thickness * tangent;
    let mut dist = 0.0;
    for i in 0..(strip.division + 1) {
        pts.push(top + dist * direction);
        pts.push(bottom + dist * direction);
        normals.push(strip.normal);
        normals.push(strip.normal);
        dist += segment;

        let alpha = (i as f32) / (strip.division as f32);
        let color = glm::mix(&strip.start_color, &strip.end_color, alpha);
        colors.push(color);
        colors.push(color);
    }

    let vertices = strip_to_vertices(pts, normals, colors);

    return vertices;
}



pub fn vertices_to_float(vertices: Vec<Vertex>) -> Vec<f32>
{
    let mut arr: Vec<f32> = Vec::new();
    for i in 0..vertices.len() {
        arr.push(vertices[i].position.x);
        arr.push(vertices[i].position.y);
        arr.push(vertices[i].position.z);
    }
    for i in 0..vertices.len() {
        arr.push(vertices[i].normal.x);
        arr.push(vertices[i].normal.y);
        arr.push(vertices[i].normal.z);
    }
    for i in 0..vertices.len() {
        arr.push(vertices[i].color.x);
        arr.push(vertices[i].color.y);
        arr.push(vertices[i].color.z);
    }
    return arr;
}



pub fn fract_pow(val: f32, exp: i32) -> f32
{
    let res: f32 = val.powf(exp as f32);
    return res - res.floor();
}

pub fn aa_to_strip(comb: Vec<i8>, comb_count: i32, comb_index: i32) -> Vec<Vertex>
{
    let mut vertices: Vec<Vertex> = Vec::new();

    let color_a = glm::vec3(fract_pow(1.2135, comb_index), fract_pow(1.8214, comb_index), fract_pow(1.5435, comb_index));
    let color_b = glm::vec3(0.1, 0.8, 1.0);
    let mut count = 0;
    let total_count = comb.len() as f32;

    let radius: f32 = 2.0;
    let separation: f32 = 2.0 * 3.14159265 / comb_count as f32;
    let angle: f32 = comb_index as f32 * separation;
    let normal: glm::Vec3 = glm::vec3(angle.cos(), angle.sin(), 0.0);
    let pos: glm::Vec3 = radius * normal;

    let mut scale = 0.3;
    let mut thick_scale = 1.0;
    let mut s_thick = 0.1;
    let mut s_pos = 1.0 * pos;
    let mut s_dir = 1.0 * normal;
    let mut s_nor = glm::vec3(0.0, 0.0, 1.0);
    let mut s_col = glm::mix(&color_a, &color_b, 0.0);

    for _aa in comb {
        let mut aa = 1 * _aa;
        count = count + 1;
        let alpha: f32 = (count as f32) / (total_count - 1.0);
        let old_dir = 1.0 * s_dir;
        let old_nor = 1.0 * s_nor;
        let old_tan = glm::cross(&old_dir, &old_nor);
        let old_thick = 1.0 * s_thick;

        scale = 0.995 * scale;
        thick_scale = 0.9999 * thick_scale;

        let mut e_pos: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0); 
        let mut e_dir: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let mut e_nor: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let mut e_col: glm::Vec3 = glm::mix(&color_a, &color_b, alpha);
        let mut e_thick = 0.05;
        let mut div: i32 = 2;
        let mut ok: bool = true;
        let mut changed: bool = false;
        if aa < 0 {
            changed = true;
            aa = -aa;
        }

        if aa == 0 {
            e_pos = s_pos + scale * old_dir;
            e_dir = old_dir;
            e_nor = glm::normalize(&(old_nor + old_tan));
            e_thick = thick_scale * old_thick;
            div = 2;
        }
        else if aa == 1 {
            e_pos = s_pos + scale * old_dir;
            e_dir = old_dir;
            e_nor = glm::normalize(&(old_nor - old_tan));
            e_thick = thick_scale * old_thick;
            div = 2;
        }
        else if aa == 2 {
            e_pos = s_pos + scale * old_dir;
            e_dir = old_dir;
            e_nor = old_tan;
            e_thick = thick_scale * old_thick;
            div = 2;
        }
        else if aa == 3 {
            e_pos = s_pos + scale * old_dir;
            e_dir = old_dir;
            e_nor = -old_tan;
            e_thick = thick_scale * old_thick;
            div = 2;
        }

        else if aa == 4 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_nor);
            e_dir = old_nor;
            e_nor = glm::normalize(&(-old_dir + 0.5 * old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 5 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_nor);
            e_dir = old_nor;
            e_nor = glm::normalize(&(-old_dir - 0.5 * old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 6 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_nor);
            e_dir = old_nor;
            e_nor = glm::normalize(&(-old_dir + old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 7 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_nor);
            e_dir = old_nor;
            e_nor = glm::normalize(&(-old_dir - old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }

        else if aa == 8 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_nor);
            e_dir = -old_nor;
            e_nor = glm::normalize(&(old_dir - 0.5 * old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 9 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_nor);
            e_dir = -old_nor;
            e_nor = glm::normalize(&(old_dir + 0.5 * old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 10 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_nor);
            e_dir = -old_nor;
            e_nor = glm::normalize(&(old_dir - old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 11 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_nor);
            e_dir = -old_nor;
            e_nor = glm::normalize(&(old_dir + old_tan));
            e_thick = thick_scale * old_thick;
            div = 6;
        }

        else if aa == 12 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_tan);
            e_dir = old_tan;
            e_nor = glm::normalize(&(old_nor + 0.5 * old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 13 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_tan);
            e_dir = old_tan;
            e_nor = glm::normalize(&(old_nor - 0.5 * old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 14 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_tan);
            e_dir = old_tan;
            e_nor = glm::normalize(&(old_nor + old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 15 {
            e_pos = s_pos + scale * (0.5 * old_dir + 0.5 * old_tan);
            e_dir = old_tan;
            e_nor = glm::normalize(&(old_nor - old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }

        else if aa == 16 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_tan);
            e_dir = -old_tan;
            e_nor = glm::normalize(&(old_nor + 0.5 * old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 17 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_tan);
            e_dir = -old_tan;
            e_nor = glm::normalize(&(old_nor - 0.5 * old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 18 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_tan);
            e_dir = -old_tan;
            e_nor = glm::normalize(&(old_nor + old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }
        else if aa == 19 {
            e_pos = s_pos + scale * (0.5 * old_dir - 0.5 * old_tan);
            e_dir = -old_tan;
            e_nor = glm::normalize(&(old_nor - old_dir));
            e_thick = thick_scale * old_thick;
            div = 6;
        }


        else {
            ok = false;
        }

        if changed {
            e_col = glm::vec3(1.0, 0.0, 0.0);
        }

        if ok
        {
            let strip: ComplexStrip = ComplexStrip {
                start_position: [s_pos.x, s_pos.y, s_pos.z],
                start_normal: [s_nor.x, s_nor.y, s_nor.z],
                start_direction: [0.5 * scale * s_dir.x, 0.5 * scale * s_dir.y, 0.5 * scale * s_dir.z],
                start_color: [s_col.x, s_col.y, s_col.z],
    
                end_position: [e_pos.x, e_pos.y, e_pos.z],
                end_normal: [e_nor.x, e_nor.y, e_nor.z],
                end_direction: [0.5 * scale * e_dir.x, 0.5 * scale * e_dir.y, 0.5 * scale * e_dir.z],
                end_color: [e_col.x, e_col.y, e_col.z],
    
                start_thickness: s_thick,
                end_thickness: e_thick,
                division: div,
            };
            vertices.append(& mut complex_strip(strip));

            s_pos = e_pos;
            s_dir = e_dir;
            s_nor = e_nor;
            s_col = e_col;
            s_thick = e_thick;
        }
    }

    return vertices;
}



pub fn dna_to_aa(seq: &str) -> Vec<Vec<i8>>
{
    let mut comb_ready: bool = false;
    let mut enc: Vec<Vec<i8>> = Vec::new();
    let mut cur_comb: Vec<i8> = Vec::new();

    let mut start_pos: usize = 0;
    let mut end_pos: usize = 0;
    let mut prot_count: i32 = 0;

    let mut it = seq.chars();
    let mut count = 0;
    let mut triple = ['a', 'a', 'a'];
    for i in 0..seq.len() {

        let m = it.next().unwrap();
        if m != '\n' && m != '\r'
        {
            triple[count] = m;
            count += 1;
        }

        if count == 3
        {
//            console_log!("{:?}", triple);
            count = 0;

            let m = triple[0];
            let n = triple[1];
            let o = triple[2];

            let mut codon: i8 = 0;
            if m == 't' {
                if n == 't' {
                    if o == 't' || o == 'c' {codon = 0;}
                    else if o == 'a' || o == 'g' {codon = 1;}
                }
                else if n == 'c' {codon = 2;}
                else if n == 'a' {
                    if o == 't' || o == 'c' {codon = 3;}
                    else if o == 'a' || o == 'g' {codon = 127;}
                }
                else if n == 'g' {
                    if o == 't' || o == 'c' {codon = 4;}
                    else if o == 'g' {codon = 5;}
                    else if o == 'a' {codon = 127;}
                }
            }
            else if m == 'c' {
                if n == 't' {codon = 1;}
                else if n == 'c' {codon = 6;}
                else if n == 'a' {
                    if o == 't' || o == 'c' {codon = 7;}
                    else if o == 'a' || o == 'g' {codon = 8;}
                }
                else if n == 'g' {codon = 9;}
            }
            else if m == 'a' {
                if n == 't' {
                    if o == 'g' {codon = 11;}
                    else {codon = 10;}
                }
                else if n == 'c' {codon = 12;}
                else if n == 'a' {
                    if o == 't' || o == 'c' {codon = 13;}
                    else if o == 'a' || o == 'g' {codon = 14;}
                }
                else if n == 'g' {
                    if o == 't' || o == 'c' {codon = 2;}
                    else if o == 'a' || o == 'g' {codon = 9;}
                }
            }
            else if m == 'g' {
                if n == 't' {codon = 15;}
                else if n == 'c' {codon = 16;}
                else if n == 'a' {
                    if o == 't' || o == 'c' {codon = 17;}
                    else if o == 'a' || o == 'g' {codon = 18;}
                }
                else if n == 'g' {codon = 19;}
            }

            if comb_ready {
                if codon == 127 {
                    if cur_comb.len() > 0 {
                        end_pos = i;
                        console_log!("protein {} - from {} to {}", prot_count, start_pos, end_pos);
//                        console_log!("{:?}", cur_comb);
                        enc.push(cur_comb);
                        cur_comb = Vec::new();
                        prot_count += 1;
                    }
                    comb_ready = false;
                }
                else {
                    cur_comb.push(codon);
                }
            }
            else if codon == 11 {
                comb_ready = true;
                start_pos = i;
            }
        }
    }

//    console_log!("{:?}", enc);
    console_log!("parts count: {}", enc.len());

    return enc;
}

pub fn compare_aa(data: Vec<i8>, reference: Vec<i8>) -> Vec<i8>
{
    let ref_len = reference.len();
    let mut result: Vec<i8> = Vec::new();
    for i in 0..data.len() {
        let val = data[i].clone();
        let mut ref_val: i8 = -1;
        if i < ref_len {ref_val = reference[i].clone();}

        if val != ref_val {
            result.push(-val);
        }
        else {
            result.push(val);
        }
    }

//    console_log!("{:?}", result);
    return result;
}


#[wasm_bindgen]
pub fn load_sequence(seq: &str, ref_seq: &str) -> Vec<f32>
{
    // convert data and references sequences to amino acids and separate into several lists based on START and STOP codons
    let ref_enc = dna_to_aa(ref_seq);
    let enc = dna_to_aa(seq);
    
    // compare the two lists of amino acids to determine what should be red
    let mut data: Vec<Vec<i8>> = Vec::new();
    for i in 0..enc.len() {
        if i < ref_enc.len() {
            let enc_i = enc[i].clone(); 
            let ref_i = ref_enc[i].clone();
            let res = compare_aa(enc_i, ref_i);
            data.push(res);
        }
        else {
            let enc_i = enc[i].clone(); 
            let ref_i: Vec<i8> = vec![-1; enc_i.len()];
            let res = compare_aa(enc_i, ref_i);
            data.push(res);
        }
    }

    // convert each amino acid list into a strip of vertices
    let mut vertices: Vec<Vertex> = Vec::new();
    for i in 0..data.len() {
        let vtx: Vec<i8> = data[i].clone();
        vertices.append(& mut aa_to_strip(vtx, data.len() as i32, i as i32));
    }

    // Separate the vertex lists into positions, normals and colors
    let arr = vertices_to_float(vertices);
    return arr;
}