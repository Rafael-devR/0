use macroquad::prelude::*;

const CENTRO: Vec3 = vec3(0.0, 2.0, 0.0);
const SENSIBILIDADE: f32 = 0.01;

fn conf() -> Conf {
    Conf {
        window_title: "Cubo 3D - arraste com o mouse".to_owned(),
        window_width: 1000,
        window_height: 700,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // Orientação do cubo como quaternion (evita gimbal lock)
    let mut rot = Quat::from_euler(EulerRot::YXZ, 0.6, 0.4, 0.0);
    let mut ultimo = Vec2::from(mouse_position());

    // 8 vértices de um cubo de lado 2, centrado na origem
    let base: [Vec3; 8] = [
        vec3(-1.0, -1.0, -1.0),
        vec3(1.0, -1.0, -1.0),
        vec3(1.0, 1.0, -1.0),
        vec3(-1.0, 1.0, -1.0),
        vec3(-1.0, -1.0, 1.0),
        vec3(1.0, -1.0, 1.0),
        vec3(1.0, 1.0, 1.0),
        vec3(-1.0, 1.0, 1.0),
    ];

    // Cada face: 4 índices de vértices + cor
    let faces: [([usize; 4], Color); 6] = [
        ([0, 1, 2, 3], RED),
        ([5, 4, 7, 6], ORANGE),
        ([4, 0, 3, 7], GREEN),
        ([1, 5, 6, 2], BLUE),
        ([3, 2, 6, 7], YELLOW),
        ([4, 5, 1, 0], PURPLE),
    ];

    // Arestas para o contorno
    let arestas: [(usize, usize); 12] = [
        (0, 1), (1, 2), (2, 3), (3, 0),
        (4, 5), (5, 6), (6, 7), (7, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];

    loop {
        // --- Entrada do mouse: arrastar com botão esquerdo gira o cubo ---
        let atual = Vec2::from(mouse_position());
        if is_mouse_button_down(MouseButton::Left) {
            let d = atual - ultimo;
            // Gira em torno dos eixos da tela (Y = horizontal, X = vertical)
            let q = Quat::from_rotation_y(d.x * SENSIBILIDADE)
                * Quat::from_rotation_x(d.y * SENSIBILIDADE);
            rot = (q * rot).normalize();
        }
        ultimo = atual;

        // Tecla R reseta a rotação
        if is_key_pressed(KeyCode::R) {
            rot = Quat::IDENTITY;
        }

        // --- Renderização ---
        clear_background(Color::new(0.08, 0.09, 0.12, 1.0));

        set_camera(&Camera3D {
            position: vec3(0.0, 5.0, 10.0),
            target: CENTRO,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0_f32.to_radians(),
            ..Default::default()
        });

        // Grade no chão para dar noção de profundidade
        draw_grid(30, 1.0, Color::new(0.6, 0.6, 0.7, 1.0), Color::new(0.25, 0.25, 0.3, 1.0));

        // Vértices transformados (rotação + posição)
        let v: Vec<Vec3> = base.iter().map(|p| rot * *p + CENTRO).collect();

        // Faces preenchidas
        let mut vertices = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        for (idx, cor) in faces.iter() {
            let ini = vertices.len() as u16;
            for &i in idx {
                vertices.push(Vertex::new(v[i].x, v[i].y, v[i].z, 0.0, 0.0, *cor));
            }
            indices.extend_from_slice(&[ini, ini + 1, ini + 2, ini, ini + 2, ini + 3]);
        }
        draw_mesh(&Mesh { vertices, indices, texture: None });

        // Arestas pretas
        for (a, b) in arestas.iter() {
            draw_line_3d(v[*a], v[*b], BLACK);
        }

        // Sombra simples projetada no chão (ajuda na profundidade)
        for (idx, _) in faces.iter() {
            let p: Vec<Vec3> = idx.iter().map(|&i| vec3(v[i].x, 0.01, v[i].z)).collect();
            draw_triangle_3d_flat(p[0], p[1], p[2]);
            draw_triangle_3d_flat(p[0], p[2], p[3]);
        }

        // --- Interface 2D ---
        set_default_camera();
        draw_text("Arraste com o botao esquerdo para girar | R: resetar", 20.0, 30.0, 26.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 58.0, 22.0, LIGHTGRAY);

        next_frame().await;
    }
}

fn draw_triangle_3d_flat(a: Vec3, b: Vec3, c: Vec3) {
    let cor = Color::new(0.0, 0.0, 0.0, 0.25);
    let mesh = Mesh {
        vertices: vec![
            Vertex::new(a.x, a.y, a.z, 0.0, 0.0, cor),
            Vertex::new(b.x, b.y, b.z, 0.0, 0.0, cor),
            Vertex::new(c.x, c.y, c.z, 0.0, 0.0, cor),
        ],
        indices: vec![0, 1, 2],
        texture: None,
    };
    draw_mesh(&mesh);
}
