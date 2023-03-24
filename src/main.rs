

fn trimesh_abuser()
{
    use tri_mesh::Mesh;
    use three_d_asset::io::*;
    use three_d_asset::{Texture2D, Model};

    // let model: three_d_asset::Model =
    // three_d_asset::io::load_and_deserialize("assets/apina2.obj").expect("Failed loading asset");
    // println!("{:?}",model);
    let mut assets = load(&["/home/turist/three_d_check_loader/assets/bricks-wall-png-4.png", "/home/turist/three_d_check_loader/assets/test_monkey.gltf"]).unwrap();
    let _texture: Texture2D = assets.deserialize("/home/turist/three_d_check_loader/assets/bricks-wall-png-4.png").unwrap();
    let model: Model = assets.deserialize("/home/turist/three_d_check_loader/assets/test_monkey.gltf").unwrap();
    //println!("{:?}", model);

    let  mesh = Mesh::new(&model.geometries[0]);
    for face in mesh.face_iter(){
        println!("{:?}=> {:?}", face, mesh.face_normal(face));
    }

    for mat in model.materials{
        println!("{:?}", mat);
    }


    //let mesh = Mesh::new(&model.geometries[0]);
}

pub(crate) fn main() {

    trimesh_abuser();

    return;

}
