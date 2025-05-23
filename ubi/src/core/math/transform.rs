// A 3*3 matrix
#[repr(C)]
pub struct Mat3([f32; 9]);

impl Mat3 {
    // build a new identity matrix
    pub fn new() -> Self {
        Mat3([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }

    // returns a pointer to the matrix, readable by OpenGL
    pub fn ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub fn mult(&mut self, mat: Mat3) {
        *self = Mat3([
            mat.0[0] * self.0[0] + mat.0[1] * self.0[3] + mat.0[2] * self.0[6],
            mat.0[0] * self.0[1] + mat.0[1] * self.0[4] + mat.0[2] * self.0[7],
            mat.0[0] * self.0[2] + mat.0[1] * self.0[5] + mat.0[2] * self.0[8],
            mat.0[3] * self.0[0] + mat.0[4] * self.0[3] + mat.0[5] * self.0[6],
            mat.0[3] * self.0[1] + mat.0[4] * self.0[4] + mat.0[5] * self.0[7],
            mat.0[3] * self.0[2] + mat.0[4] * self.0[5] + mat.0[5] * self.0[8],
            mat.0[6] * self.0[0] + mat.0[7] * self.0[3] + mat.0[8] * self.0[6],
            mat.0[6] * self.0[1] + mat.0[7] * self.0[4] + mat.0[8] * self.0[7],
            mat.0[6] * self.0[2] + mat.0[7] * self.0[5] + mat.0[8] * self.0[8],
        ])
    }

    // Add a scale transforamtion to the mat3, for each axis
    // the scale is center is (0.0, 0.0)
    pub fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.mult(Mat3([x_scale, 0.0, 0.0, 0.0, y_scale, 0.0, 0.0, 0.0, 1.0]));
    }

    // Add a rotation transformation to the mat3 clockwise areound (0.0, 0.0)
    pub fn rotate(&mut self, angle: f32) {
        self.mult(Mat3([
            angle.cos(),
            angle.sin(),
            0.0,
            -angle.sin(),
            angle.cos(),
            0.0,
            0.0,
            0.0,
            1.0,
        ]));
    }

    // Add a translation trnasformation
    pub fn translate(&mut self, x_move: f32, y_move: f32) {
        self.mult(Mat3([0.0, 0.0, x_move, 0.0, 0.0, y_move, 0.0, 0.0, 1.0]));
    }
}

// A 4*4 matrix
#[repr(C)]
pub struct Mat4([f32; 16]);

impl Mat4 {
    // build a new identity matrix
    pub fn new() -> Self {
        Mat4([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }

    // returns a pointer to the matrix, readable by OpenGL
    pub fn ptr(&self) -> *const f32 {
        self.0.as_ptr()
    }

    pub fn mult(&mut self, mat: Mat4) {
        *self = Mat4([
            mat.0[0] * self.0[0]
                + mat.0[1] * self.0[4]
                + mat.0[2] * self.0[8]
                + mat.0[3] * self.0[12],
            mat.0[0] * self.0[1]
                + mat.0[1] * self.0[5]
                + mat.0[2] * self.0[9]
                + mat.0[3] * self.0[13],
            mat.0[0] * self.0[2]
                + mat.0[1] * self.0[6]
                + mat.0[2] * self.0[10]
                + mat.0[3] * self.0[14],
            mat.0[0] * self.0[3]
                + mat.0[1] * self.0[7]
                + mat.0[2] * self.0[11]
                + mat.0[3] * self.0[15],
            mat.0[4] * self.0[0]
                + mat.0[5] * self.0[4]
                + mat.0[6] * self.0[8]
                + mat.0[7] * self.0[12],
            mat.0[4] * self.0[1]
                + mat.0[5] * self.0[5]
                + mat.0[6] * self.0[9]
                + mat.0[7] * self.0[13],
            mat.0[4] * self.0[2]
                + mat.0[5] * self.0[6]
                + mat.0[6] * self.0[10]
                + mat.0[7] * self.0[14],
            mat.0[4] * self.0[3]
                + mat.0[5] * self.0[7]
                + mat.0[6] * self.0[11]
                + mat.0[7] * self.0[15],
            mat.0[8] * self.0[0]
                + mat.0[9] * self.0[4]
                + mat.0[10] * self.0[8]
                + mat.0[11] * self.0[12],
            mat.0[8] * self.0[1]
                + mat.0[9] * self.0[5]
                + mat.0[10] * self.0[9]
                + mat.0[11] * self.0[13],
            mat.0[8] * self.0[2]
                + mat.0[9] * self.0[6]
                + mat.0[10] * self.0[10]
                + mat.0[11] * self.0[14],
            mat.0[8] * self.0[3]
                + mat.0[9] * self.0[7]
                + mat.0[10] * self.0[11]
                + mat.0[11] * self.0[15],
            mat.0[12] * self.0[0]
                + mat.0[13] * self.0[4]
                + mat.0[14] * self.0[8]
                + mat.0[15] * self.0[12],
            mat.0[12] * self.0[1]
                + mat.0[13] * self.0[5]
                + mat.0[14] * self.0[9]
                + mat.0[15] * self.0[13],
            mat.0[12] * self.0[2]
                + mat.0[13] * self.0[6]
                + mat.0[14] * self.0[10]
                + mat.0[15] * self.0[14],
            mat.0[12] * self.0[3]
                + mat.0[13] * self.0[7]
                + mat.0[14] * self.0[11]
                + mat.0[15] * self.0[15],
        ])
    }

    // Add a scale transforamtion to the mat3, for each axis
    // the scale is center is (0.0, 0.0)
    pub fn scale(&mut self, x_scale: f32, y_scale: f32, z_scale: f32) {
        self.mult(Mat4([
            x_scale, 0.0, 0.0, 0.0, 0.0, y_scale, 0.0, 0.0, 0.0, 0.0, z_scale, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the X axis, clockiwse.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_x(&mut self, angle: f32) {
        self.mult(Mat4([
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            angle.cos(),
            angle.sin(),
            0.0,
            0.0,
            -angle.sin(),
            angle.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the Y axis, clockiwse.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_y(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos(),
            0.0,
            angle.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -angle.sin(),
            0.0,
            angle.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]));
    }

    /// Add a rotation transformation to the Mat4 around the Z axis, clockiwse.
    /// The rotation center is (0.0, 0.0, 0.0).
    pub fn rotate_z(&mut self, angle: f32) {
        self.mult(Mat4([
            angle.cos(),
            angle.sin(),
            0.0,
            0.0,
            -angle.sin(),
            angle.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]));
    }

    // Add a translation trnasformation
    pub fn translate(&mut self, x_move: f32, y_move: f32, z_move: f32) {
        self.mult(Mat4([
            1.0, 0.0, 0.0, x_move, 0.0, 1.0, 0.0, y_move, 0.0, 0.0, 1.0, z_move, 0.0, 0.0, 0.0, 1.0,
        ]));
    }

    /// For view matrix. Moves the "camera" to (eye_x, eye_y, eye_z), looking at (target_x, target_y, target_z), with the up direction pointing to (up_x, up_y, up_z).
    /// Replaces any earlier transformation to this Mat4.
    pub fn lookat(
        &mut self,
        eye_x: f32,
        eye_y: f32,
        eye_z: f32,
        target_x: f32,
        target_y: f32,
        target_z: f32,
        mut up_x: f32,
        mut up_y: f32,
        mut up_z: f32,
    ) {
        // Forward vector
        let (mut f_x, mut f_y, mut f_z) = (eye_x - target_x, eye_y - target_y, eye_z - target_z);
        let invlen = 1.0 / (f_x * f_x + f_y * f_y + f_z * f_z).sqrt();
        (f_x, f_y, f_z) = (f_x * invlen, f_y * invlen, f_z * invlen);

        // Left vector
        let (mut l_x, mut l_y, mut l_z) = (
            up_y * f_z - up_z * f_y,
            up_z * f_x - up_x * f_z,
            up_x * f_y - up_y * f_x,
        );
        let invlen = 1.0 / (l_x * l_x + l_y * l_y + l_z * l_z).sqrt();
        (l_x, l_y, l_z) = (l_x * invlen, l_y * invlen, l_z * invlen);

        // Up vector correction
        (up_x, up_y, up_z) = (
            f_y * l_z - f_z * l_y,
            f_z * l_x - f_x * l_z,
            f_x * l_y - f_y * l_x,
        );

        *self = Self::new();
        self.translate(-eye_x, -eye_y, -eye_z);
        self.mult(Mat4([
            l_x, l_y, l_z, 0.0, up_x, up_y, up_z, 0.0, f_x, f_y, f_z, 0.0, 0.0, 0.0, 0.0,
            1.0,
            // Transposed from:
            // l_x ,   up_x,   f_x ,   0.0 ,
            // l_y ,   up_y,   f_y ,   0.0 ,
            // l_z ,   up_z,   f_z ,   0.0 ,
            // 0.0 ,   0.0 ,   0.0 ,   1.0 ,
        ]));
    }
}
