

class Scene {
    constructor()
    {
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color().setHSL( 0.6, 0, 1 );
    
        this.camera = new THREE.PerspectiveCamera( 75, window.innerWidth / window.innerHeight, 0.1, 1000 );
    
        this.renderer = new THREE.WebGLRenderer();
        this.renderer.physicallyCorrectLights = true;
        this.renderer.outputEncoding = THREE.sRGBEncoding;
        this.renderer.shadowMap.enabled = true;
        this.renderer.toneMapping = THREE.ReinhardToneMapping;
        this.renderer.setPixelRatio( window.devicePixelRatio );
        this.renderer.setSize( window.innerWidth, window.innerHeight );
        document.body.appendChild( this.renderer.domElement );
        this.renderer.domElement.style.top = "0";
        this.renderer.domElement.style.left = "0";
        this.renderer.domElement.style.position = "absolute";
    
    
        const hemiLight = new THREE.HemisphereLight( 0xffffff, 0xffffff, 0.6 );
        hemiLight.color.setHSL( 0.6, 1, 0.6 );
        hemiLight.groundColor.setHSL( 0.095, 1, 0.75 );
        hemiLight.position.set( 0, 10, 0 );
        hemiLight.visible = true;
        this.scene.add( hemiLight );
    
        const ptlight0 = new THREE.PointLight( 0xffffff, 100, 0 );
        ptlight0.position.set( 5, 5, 5 );
        this.scene.add( ptlight0 );
    
        const ptlight1 = new THREE.PointLight( 0xffffff, 100, 0 );
        ptlight1.position.set( -5, 5, -5 );
        this.scene.add( ptlight1 );
    
    
        this.camera.position.z = 5;
    }


    AddGeometry(vertices)
    {
        let size = vertices.length / 3;
        let positions = vertices.subarray(0, size);
        let normals = vertices.subarray(size, 2 * size);
        let colors = vertices.subarray(2 * size, 3 * size);

        const geometry = new THREE.BufferGeometry();
        geometry.setAttribute( 'position', new THREE.BufferAttribute( positions, 3 ) );
        geometry.setAttribute( 'normal', new THREE.BufferAttribute( normals, 3 ) );
        geometry.setAttribute( 'color', new THREE.BufferAttribute( colors, 3 ) );

        const material = new THREE.MeshStandardMaterial( {
                color: 0xffffff,
                roughness: 0.5,
                metalness: 0.1,
                vertexColors: true,
                side: THREE.DoubleSide
        } );

        const mesh = new THREE.Mesh( geometry, material );
        this.scene.add( mesh );

        return mesh;
    }

    Animate(obj)
    {
        let scene = this.scene;
        let camera = this.camera;
        let renderer = this.renderer;
        function animate() {
            requestAnimationFrame( animate );
            obj.rotation.x += 0.01;
            obj.rotation.y += 0.01;
            renderer.render( scene, camera );
        }
        animate();
    }
}




async function LoadSequence(url)
{
    var res = await fetch(url);
    var data = await res.text();
    return data;
}