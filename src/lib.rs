use std::{rc::Rc, cell::RefCell, mem::size_of};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram, HtmlCanvasElement, HtmlInputElement};
extern crate js_sys;
extern crate nalgebra_glm as glm;

mod obj_parse;
use obj_parse::obj::parse;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/web/src/lib.ts")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn read_file(path: &str) -> Result<String, JsValue>;
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn context(canvas: HtmlCanvasElement) -> WebGl2RenderingContext {
    canvas.get_context("webgl2")
        .expect("no context named `webgl2` found for canvas")
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .expect("failed dyn_into WebGl2RenderingContext")
}

fn get_canvas() -> HtmlCanvasElement {
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    return canvas;
}

fn get_window_dimmensions() -> (i32, i32) {
    (window().inner_width().unwrap().as_f64().unwrap() as i32,
    window().inner_height().unwrap().as_f64().unwrap() as i32)
}

fn resize_callback(context_ref: WebGl2RenderingContext, canvas_ref: HtmlCanvasElement) {
    let (width, height) = get_window_dimmensions();
    canvas_ref.set_width(width as u32);
    canvas_ref.set_height(height as u32);
    context_ref.viewport(0, 0, width, height);
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = document();
    let canvas = get_canvas();
    let context = context(canvas.clone());

    let vertex_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("./shaders/vertex.glsl")
    )?;
    let fragment_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("./shaders/fragment.glsl")
    )?;
    let program = link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let obj_data = read_file("dragon.obj").unwrap();
    // let obj_data = read_file("hous.obj").unwrap();
    let (verts_vec, indices_vec) = parse(&obj_data);
    let vertices = verts_vec.as_slice();
    let indices = indices_vec.as_slice();

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let texcoord_attribute_location = context.get_attrib_location(&program, "texcoord");
    let normal_attribute_location = context.get_attrib_location(&program, "normal");
    
    let vbo = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let ebo = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));

    unsafe {
        let indices_array_buf_view = js_sys::Uint16Array::view(&indices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    let floatsize: i32 = size_of::<f32>() as i32;
    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        8 * floatsize,
        0,
    );
    context.vertex_attrib_pointer_with_i32(
        texcoord_attribute_location as u32,
        2,
        WebGl2RenderingContext::FLOAT,
        false,
        8 * floatsize,
        3 * floatsize,
    );
    context.vertex_attrib_pointer_with_i32(
        normal_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        8 * floatsize,
        5 * floatsize,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);
    context.enable_vertex_attrib_array(texcoord_attribute_location as u32);
    context.enable_vertex_attrib_array(normal_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));
    context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));

    // Window resize callback
    let resize_func = Closure::wrap(Box::new({
        let context_ref = context.clone();
        let canvas_ref = canvas.clone();
        move || {
            resize_callback(context_ref.clone(), canvas_ref.clone());
        }
    }) as Box<dyn FnMut()>);
    window().add_event_listener_with_callback("resize", resize_func.as_ref().unchecked_ref())?;
    resize_callback(context.clone(), canvas.clone());
    resize_func.forget();

    // let view = RefCell::new(glm::identity::<f32, 4>());
    // *view.borrow_mut() = glm::translate(&view.borrow(), &glm::vec3(0.0, 0.0, -3.0));
    // view = glm::translate(&view, &glm::vec3(0.0, 0.0, -1.0 * (1.0 - f32::exp(-1.0 * 1.0 * 3.0))));

    // Window keyboard callback
    // let keydown_func = Closure::wrap(Box::new({
    //     // let context_ref = context.clone();
    //     // let canvas_ref = canvas.clone();
    //     let mut ref_view = view.clone().borrow_mut();
    //     move |event: web_sys::KeyboardEvent| {
    //         match event.code().as_str() {
    //             "KeyW" => {
    //                 *ref_view = glm::translate(&ref_view, &glm::vec3(0.0, 0.0, -1000.0));
    //             },
    //             _ => log(format!("{}", event.code().as_str()).as_str()),
    //         }
    //     }
    // }) as Box<dyn FnMut(_)>);
    // document.add_event_listener_with_callback("keydown", keydown_func.as_ref().unchecked_ref())?;
    // keydown_func.forget();

    // This is our render loop
    let render_func = Rc::new(RefCell::new(None));
    let ref_render_func = render_func.clone();

    *ref_render_func.borrow_mut() = Some(Closure::wrap(Box::new({
        let context_ref = context.clone();
        let indices_len = indices.len() as i32;
        move || {
            let time = window().performance().unwrap().now() as f32;

            let toggle1 = document.get_element_by_id("hyper").unwrap();
            let toggle: web_sys::HtmlInputElement = toggle1.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let checked = toggle.checked();

            let mut model = glm::identity::<f32, 4>();
            model = glm::scale(&model, &glm::vec3(0.2, 0.2, 0.2));
            model = glm::translate(&model, &glm::vec3(0.0, -4.0, -2.0));
            model = glm::rotate(&model, 
                f32::to_radians(time / 70.0), 
                &glm::vec3(0.0, 1.0, 0.0)
            );

            let mut view = glm::identity::<f32, 4>();
            view = glm::translate(&view, &glm::vec3(0.0, 0.3 + f32::sin(time / 600.0) * 0.5, -3.0));
            // view = glm::translate(&view, &glm::vec3(0.0, 0.0, -1.0 * (1.0 - f32::exp(-1.0 * 1.0 * 3.0))));

            let (width, height) = get_window_dimmensions();
            let aspect = width as f32 / height as f32;
            let projection;
            projection = glm::perspective(aspect, f32::to_radians(45.0), 0.1, 100.0);

            let model_location = context_ref.get_uniform_location(&program, "model").unwrap();
            let view_location = context_ref.get_uniform_location(&program, "view").unwrap();
            let proj_location = context_ref.get_uniform_location(&program, "projection").unwrap();
            context_ref.uniform_matrix4fv_with_f32_array(Some(&model_location), false, model.as_slice());
            context_ref.uniform_matrix4fv_with_f32_array(Some(&view_location), false, view.as_slice());
            context_ref.uniform_matrix4fv_with_f32_array(Some(&proj_location), false, projection.as_slice());
           
            let bool_location = context_ref.get_uniform_location(&program, "toggle").unwrap();
            context_ref.uniform1i(Some(&bool_location), if checked {1} else {0});
            let time_location = context_ref.get_uniform_location(&program, "time").unwrap();
            context_ref.uniform1f(Some(&time_location), time);

            draw(&context_ref, indices_len);
           
            let timeout_func = Closure::wrap(Box::new({
                let ref_render_func = render_func.clone();
                move || {
                    request_animation_frame(ref_render_func.borrow().as_ref().unwrap());
                }
            }) as Box<dyn FnMut()>);
            let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(timeout_func.as_ref().unchecked_ref(), 83);
            timeout_func.forget();
        }
    }) as Box<dyn FnMut()>));
    request_animation_frame(ref_render_func.borrow().as_ref().unwrap());

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, indices_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_elements_with_i32(
        WebGl2RenderingContext::TRIANGLES,
        indices_count,
        WebGl2RenderingContext::UNSIGNED_SHORT,
        0
    );
    // context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vertex_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
