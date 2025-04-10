pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let container_area = rectangle_area(x, y);
    
    let single_object_area = match objects {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };
    
    let total_area_needed = single_object_area * times as f64;
    
    total_area_needed <= container_area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let container_volume = parallelepiped_volume(x, y, z);
    
    let single_object_volume = match objects {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Pyramid => {
            let base_area = a as f64; 
            triangular_pyramid_volume(base_area, b)
        },
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };
    
    let total_volume_needed = single_object_volume * times as f64;
    
    total_volume_needed <= container_volume as f64
}