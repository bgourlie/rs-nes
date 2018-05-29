#![recursion_limit = "2048"]

#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use stdweb::web::IParentNode;
use stdweb::web::FileList;

use stdweb::web::{
    self, ArrayBuffer, Element, FileReader, FileReaderResult, IElement, IEventTarget, INode,
};

use stdweb::web::event::{ChangeEvent, ClickEvent, IEvent, KeyboardLocation, ProgressLoadEvent};

use stdweb::unstable::TryInto;
use stdweb::web::html_element::InputElement;
use stdweb::{Once, Value};

macro_rules! enclose {
    ( [$( $x:ident ),*] $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

struct RsNesWeb;

// This creates a really basic WebGL context for blitting a single texture.
// On some web browsers this is faster than using a 2d canvas.
fn setup_webgl(canvas: &Element) -> Value {
    const FRAGMENT_SHADER: &'static str = r#"
        precision mediump float;
        varying vec2 v_texcoord;
        uniform sampler2D u_sampler;
        void main() {
            gl_FragColor = vec4( texture2D( u_sampler, vec2( v_texcoord.s, v_texcoord.t ) ).rgb, 1.0 );
        }
    "#;

    const VERTEX_SHADER: &'static str = r#"
        attribute vec2 a_position;
        attribute vec2 a_texcoord;
        uniform mat4 u_matrix;
        varying vec2 v_texcoord;
        void main() {
            gl_Position = u_matrix * vec4( a_position, 0.0, 1.0 );
            v_texcoord = a_texcoord;
        }
    "#;

    fn ortho(left: f64, right: f64, bottom: f64, top: f64) -> Vec<f64> {
        let mut m = vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];

        m[0 * 4 + 0] = 2.0 / (right - left);
        m[1 * 4 + 1] = 2.0 / (top - bottom);
        m[3 * 4 + 0] = (right + left) / (right - left) * -1.0;
        m[3 * 4 + 1] = (top + bottom) / (top - bottom) * -1.0;

        return m;
    }

    js!(
        var gl;
        var webgl_names = ["webgl", "experimental-webgl", "webkit-3d", "moz-webgl"];
        for( var i = 0; i < webgl_names.length; ++i ) {
            var name = webgl_names[ i ];
            try {
                gl = @{canvas}.getContext( name );
            } catch( err ) {}

            if( gl ) {
                console.log( "WebGL support using context:", name );
                break;
            }
        }

        if( gl === null ) {
            console.error( "WebGL rendering context not found." );
            return null;
        }

        var vertex_shader = gl.createShader( gl.VERTEX_SHADER );
        var fragment_shader = gl.createShader( gl.FRAGMENT_SHADER );
        gl.shaderSource( vertex_shader, @{VERTEX_SHADER} );
        gl.shaderSource( fragment_shader, @{FRAGMENT_SHADER} );
        gl.compileShader( vertex_shader );
        gl.compileShader( fragment_shader );

        if( !gl.getShaderParameter( vertex_shader, gl.COMPILE_STATUS ) ) {
            console.error( "WebGL vertex shader compilation failed:", gl.getShaderInfoLog( vertex_shader ) );
            return null;
        }

        if( !gl.getShaderParameter( fragment_shader, gl.COMPILE_STATUS ) ) {
            console.error( "WebGL fragment shader compilation failed:", gl.getShaderInfoLog( fragment_shader ) );
            return null;
        }

        var program = gl.createProgram();
        gl.attachShader( program, vertex_shader );
        gl.attachShader( program, fragment_shader );
        gl.linkProgram( program );
        if( !gl.getProgramParameter( program, gl.LINK_STATUS ) ) {
            console.error( "WebGL program linking failed!" );
            return null;
        }

        gl.useProgram( program );

        var vertex_attr = gl.getAttribLocation( program, "a_position" );
        var texcoord_attr = gl.getAttribLocation( program, "a_texcoord" );

        gl.enableVertexAttribArray( vertex_attr );
        gl.enableVertexAttribArray( texcoord_attr );

        var sampler_uniform = gl.getUniformLocation( program, "u_sampler" );
        gl.uniform1i( sampler_uniform, 0 );

        var matrix = @{ortho( 0.0, 256.0, 240.0, 0.0 )};
        var matrix_uniform = gl.getUniformLocation( program, "u_matrix" );
        gl.uniformMatrix4fv( matrix_uniform, false, matrix );

        var texture = gl.createTexture();
        gl.bindTexture( gl.TEXTURE_2D, texture );
        gl.texImage2D( gl.TEXTURE_2D, 0, gl.RGBA, 256, 256, 0, gl.RGBA, gl.UNSIGNED_BYTE, new Uint8Array( 256 * 256 * 4 ) );
        gl.texParameteri( gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST );
        gl.texParameteri( gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST );

        var vertex_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ARRAY_BUFFER, vertex_buffer );
        var vertices = [
            0.0, 0.0,
            0.0, 240.0,
            256.0, 0.0,
            256.0, 240.0
        ];
        gl.bufferData( gl.ARRAY_BUFFER, new Float32Array( vertices ), gl.STATIC_DRAW );
        gl.vertexAttribPointer( vertex_attr, 2, gl.FLOAT, false, 0, 0 );

        var texcoord_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ARRAY_BUFFER, texcoord_buffer );
        var texcoords = [
            0.0, 0.0,
            0.0, 240.0 / 256.0,
            1.0, 0.0,
            1.0, 240.0 / 256.0
        ];
        gl.bufferData( gl.ARRAY_BUFFER, new Float32Array( texcoords ), gl.STATIC_DRAW );
        gl.vertexAttribPointer( texcoord_attr, 2, gl.FLOAT, false, 0, 0 );

        var index_buffer = gl.createBuffer();
        gl.bindBuffer( gl.ELEMENT_ARRAY_BUFFER, index_buffer );
        var indices = [
            0, 1, 2,
            2, 3, 1
        ];
        gl.bufferData( gl.ELEMENT_ARRAY_BUFFER, new Uint16Array( indices ), gl.STATIC_DRAW );

        gl.clearColor( 0.0, 0.0, 0.0, 1.0 );
        gl.enable( gl.DEPTH_TEST );
        gl.viewport( 0, 0, 256, 240 );

        return gl;
    )
}

impl RsNesWeb {
    fn new(canvas: &Element) -> Self {
        let gl = setup_webgl(&canvas);

        let js_ctx = js!(
            var h = {};
            var canvas = @{canvas};

            h.gl = @{gl};
            h.audio = new AudioContext();
            h.empty_audio_buffers = [];
            h.play_timestamp = 0;

            if( !h.gl ) {
                console.log( "No WebGL; using Canvas API" );

                // If the WebGL **is** supported but something else
                // went wrong the web browser won't let us create
                // a normal canvas context on a WebGL-ified canvas,
                // so we recreate a new canvas here to work around that.
                var new_canvas = canvas.cloneNode( true );
                canvas.parentNode.replaceChild( new_canvas, canvas );
                canvas = new_canvas;

                h.ctx = canvas.getContext( "2d" );
                h.img = h.ctx.createImageData( 256, 240 );
                h.buffer = new Uint32Array( h.img.data.buffer );
            }

            return h;
        );

        RsNesWeb
    }

    fn draw(&mut self) {}

    fn on_key(&mut self, key: &str, location: KeyboardLocation, is_pressed: bool) -> bool {
        false
    }
}

fn emulate_for_a_single_frame() {}

fn main_loop() {}

#[derive(Deserialize)]
struct RomEntry {
    name: String,
    file: String,
}

js_deserializable!(RomEntry);

fn show(selector: &str) {
    web::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .class_list()
        .remove("hidden");
}

fn hide(selector: &str) {
    web::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .class_list()
        .add("hidden");
}

fn fetch_builtin_rom_list<F: FnOnce(Vec<RomEntry>) + 'static>(callback: F) {
    let on_rom_list_loaded = Once(move |mut roms: Vec<RomEntry>| {
        roms.sort_by(|a, b| a.name.cmp(&b.name));
        callback(roms);
    });

    js! {
        var req = new XMLHttpRequest();
        req.addEventListener( "load" , function() {
            var cb = @{on_rom_list_loaded};
            cb( JSON.parse( req.responseText ) );
            cb.drop();
        });
        req.open( "GET", "roms/index.json" );
        req.send();
    }
}

fn support_builtin_roms(roms: Vec<RomEntry>, pinky: Rc<RefCell<RsNesWeb>>) {
    let entries = web::document()
        .query_selector("#rom-list")
        .unwrap()
        .unwrap();
    for rom in roms {
        let entry = web::document().create_element("button").unwrap();
        let name = rom.name;
        let file = rom.file;

        entry.set_text_content(&name);
        entries.append_child(&entry);
        entry.add_event_listener(enclose!( [pinky] move |_: ClickEvent| {
            hide( "#change-rom-menu" );
            hide( "#side-text" );
            show( "#loading" );

            let builtin_rom_loaded = Once( enclose!( [pinky] move |array_buffer: ArrayBuffer| {
                let rom_data: Vec< u8 > = array_buffer.into();
                load_rom( &pinky, &rom_data );
            }));
            js! {
                var req = new XMLHttpRequest();
                req.addEventListener( "load" , function() {
                    @{builtin_rom_loaded}( req.response );
                });
                req.open( "GET", "roms/" + @{&file} );
                req.responseType = "arraybuffer";
                req.send();
            }
        }));
    }
}

fn support_custom_roms(pinky: Rc<RefCell<RsNesWeb>>) {
    //    let browse_for_roms_button = web::document().query_selector( "#browse-for-roms" ).unwrap().unwrap();
    //    browse_for_roms_button.add_event_listener( move |event: ChangeEvent| {
    //        let input: InputElement = event.target().unwrap().try_into().unwrap();
    //        let file: String = input.raw_value();
    //
    //        hide( "#change-rom-menu" );
    //        hide( "#side-text" );
    //        show( "#loading" );
    //
    //        let reader = FileReader::new();
    //        reader.add_event_listener( enclose!( [pinky, reader] move |_: ProgressLoadEvent| {
    //            let rom_data: Vec< u8 > = match reader.result().unwrap() {
    //                FileReaderResult::ArrayBuffer( buffer ) => buffer,
    //                _ => unreachable!()
    //            }.into();
    //
    //            load_rom( &pinky, &rom_data );
    //        }));
    //
    //        reader.read_as_array_buffer( &file );
    //    });
}

fn support_rom_changing(pinky: Rc<RefCell<RsNesWeb>>) {
    let change_rom_button = web::document()
        .query_selector("#change-rom-button")
        .unwrap()
        .unwrap();
    change_rom_button.add_event_listener(enclose!( [pinky] move |_: ClickEvent| {
        //pinky.borrow_mut().pause();
        hide( "#viewport" );
        hide( "#change-rom-button" );
        show( "#change-rom-menu" );
        show( "#rom-menu-close" );
    }));

    let rom_menu_close_button = web::document()
        .query_selector("#rom-menu-close")
        .unwrap()
        .unwrap();
    rom_menu_close_button.add_event_listener(move |_: ClickEvent| {
        //pinky.borrow_mut().unpause();
        show("#viewport");
        show("#change-rom-button");
        hide("#change-rom-menu");
        hide("#rom-menu-close");
    });
}

fn support_input(pinky: Rc<RefCell<RsNesWeb>>) {}

fn load_rom(pinky: &Rc<RefCell<RsNesWeb>>, rom_data: &[u8]) {
    hide("loading");
    hide("error");

    //    let mut pinky = pinky.borrow_mut();
    //    let pinky = pinky.deref_mut();
    //    if let Err( err ) = nes::Interface::load_rom_from_memory( pinky, rom_data ) {
    //        handle_error( err );
    //        return;
    //    }
    //    pinky.unpause();

    show("#viewport");
    show("#change-rom-button");
}

fn handle_error<E: Into<Box<Error>>>(error: E) {
    let error_message = format!("{}", error.into());
    web::document()
        .query_selector("#error-description")
        .unwrap()
        .unwrap()
        .set_text_content(&error_message);

    hide("#viewport");
    hide("#change-rom-button");
    hide("#rom-menu-close");
    show("#change-rom-menu");
    show("#error");
}

fn main() {
    stdweb::initialize();

    let canvas = web::document()
        .query_selector("#viewport")
        .unwrap()
        .unwrap();
    let pinky = Rc::new(RefCell::new(RsNesWeb::new(&canvas)));

    support_custom_roms(pinky.clone());
    support_rom_changing(pinky.clone());

    fetch_builtin_rom_list(enclose!(
        [pinky] | roms | {
            support_builtin_roms(roms, pinky);

            hide("#loading");
            show("#change-rom-menu");
        }
    ));

    //    support_input( pinky.clone() );

    //    web::window().request_animation_frame( move |_| {
    //        main_loop( pinky );
    //    });

    stdweb::event_loop();
}

/// Returns the list of selected files. **Only for inputs of type `file`**.
    #[inline]
    pub fn files( input: &InputElement ) -> Option< FileList > {
            js! (
            return @{input}.files;
        ).try_into().ok()
            }