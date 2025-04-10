pub fn area_fit(
    x: usize,
    y: usize,
    objects: crate::areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let container_area = crate::areas_volumes::rectangle_area(x, y);
    
    let single_object_area = match objects {
        crate::areas_volumes::GeometricalShapes::Square => crate::areas_volumes::square_area(a) as f64,
        crate::areas_volumes::GeometricalShapes::Circle => crate::areas_volumes::circle_area(a),
        crate::areas_volumes::GeometricalShapes::Rectangle => crate::areas_volumes::rectangle_area(a, b) as f64,
        crate::areas_volumes::GeometricalShapes::Triangle => crate::areas_volumes::triangle_area(a, b),
    };
    
    let total_area_needed = single_object_area * times as f64;
    
    total_area_needed <= container_area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: crate::areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let container_volume = crate::areas_volumes::parallelepiped_volume(x, y, z);
    
    let single_object_volume = match objects {
        crate::areas_volumes::GeometricalVolumes::Cube => crate::areas_volumes::cube_volume(a) as f64,
        crate::areas_volumes::GeometricalVolumes::Sphere => crate::areas_volumes::sphere_volume(a),
        crate::areas_volumes::GeometricalVolumes::Cone => crate::areas_volumes::cone_volume(a, b),
        crate::areas_volumes::GeometricalVolumes::Pyramid => {
            let base_area = a as f64; 
            crate::areas_volumes::triangular_pyramid_volume(base_area, b)
        },
        crate::areas_volumes::GeometricalVolumes::Parallelepiped => crate::areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };
    
    let total_volume_needed = single_object_volume * times as f64;
    
    total_volume_needed <= container_volume as f64
}