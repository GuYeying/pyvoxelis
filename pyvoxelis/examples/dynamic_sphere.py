import pygame
import numpy as np
import glm

from OpenGL.GL import *
from OpenGL.GL.shaders import compileProgram, compileShader

import pyvoxelis
import warnings

warnings.filterwarnings(
    "ignore",
    category=UserWarning,
    module="pygame.pkgdata"
)

# =========================================================
# Shader
# =========================================================

VERTEX_SHADER = """
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

out vec3 normal;

void main()
{
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    normal = aNormal;
}
"""

FRAGMENT_SHADER = """
#version 330 core

in vec3 normal;
out vec4 FragColor;

void main()
{
    vec3 n = normalize(normal);

    float r = (n.x + 1.0) * 0.45;
    float g = (n.y + 1.0) * 0.45;
    float b = (n.z + 1.0) * 0.55;

    FragColor = vec4(r, g, b, 1.0);
}
"""

# =========================================================
# Camera
# =========================================================
class Camera:

    def __init__(self):

        self.pos = glm.vec3(-0.22, 0.9, 1)

        self.yaw = -90
        self.pitch = -25

        self.speed = 10.0
        self.sensitivity = 0.15

        # 初始方向
        self.front = glm.vec3(
            0.75735,
            -0.439626,
            -0.482857
        )

        self.up = glm.vec3(0,1,0)

        self.right = glm.vec3(
            0.519817,
            0,
            0.854277
        )

        self.update()

    def update(self):

        f = glm.vec3()

        f.x = glm.cos(glm.radians(self.yaw)) * glm.cos(glm.radians(self.pitch))
        f.y = glm.sin(glm.radians(self.pitch))
        f.z = glm.sin(glm.radians(self.yaw)) * glm.cos(glm.radians(self.pitch))

        self.front = glm.normalize(f)

        self.right = glm.normalize(
            glm.cross(self.front, self.up)
        )

    def view(self):

        return glm.lookAt(
            self.pos,
            self.pos + self.front,
            self.up
        )

# =========================================================
# Dynamic Mesh
# =========================================================

class DynamicMesh:

    def __init__(self):

        self.vao = glGenVertexArrays(1)

        self.vbo_pos = glGenBuffers(1)
        self.vbo_nrm = glGenBuffers(1)

        self.ebo = glGenBuffers(1)

        self.count = 0

        glBindVertexArray(self.vao)

        # positions
        glBindBuffer(GL_ARRAY_BUFFER, self.vbo_pos)

        glBufferData(
            GL_ARRAY_BUFFER,
            1,
            None,
            GL_DYNAMIC_DRAW
        )

        glVertexAttribPointer(
            0,
            3,
            GL_FLOAT,
            GL_FALSE,
            0,
            None
        )

        glEnableVertexAttribArray(0)

        # normals
        glBindBuffer(GL_ARRAY_BUFFER, self.vbo_nrm)

        glBufferData(
            GL_ARRAY_BUFFER,
            1,
            None,
            GL_DYNAMIC_DRAW
        )

        glVertexAttribPointer(
            1,
            3,
            GL_FLOAT,
            GL_FALSE,
            0,
            None
        )

        glEnableVertexAttribArray(1)

        # indices
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo)

        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            1,
            None,
            GL_DYNAMIC_DRAW
        )

        glBindVertexArray(0)

    def update(self, vs, ns, ids):

        vs = np.array(vs, dtype=np.float32)
        ns = np.array(ns, dtype=np.float32)
        ids = np.array(ids, dtype=np.uint32)

        self.count = len(ids)

        glBindVertexArray(self.vao)

        # positions
        glBindBuffer(GL_ARRAY_BUFFER, self.vbo_pos)

        glBufferData(
            GL_ARRAY_BUFFER,
            vs.nbytes,
            vs,
            GL_DYNAMIC_DRAW
        )

        # normals
        glBindBuffer(GL_ARRAY_BUFFER, self.vbo_nrm)

        glBufferData(
            GL_ARRAY_BUFFER,
            ns.nbytes,
            ns,
            GL_DYNAMIC_DRAW
        )

        # indices
        glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.ebo)

        glBufferData(
            GL_ELEMENT_ARRAY_BUFFER,
            ids.nbytes,
            ids,
            GL_DYNAMIC_DRAW
        )

        glBindVertexArray(0)

    def draw(self):

        if self.count == 0:
            return

        glBindVertexArray(self.vao)

        glDrawElements(
            GL_TRIANGLES,
            self.count,
            GL_UNSIGNED_INT,
            None
        )

    def destroy(self):

        glDeleteVertexArrays(1, [self.vao])

        glDeleteBuffers(1, [self.vbo_pos])
        glDeleteBuffers(1, [self.vbo_nrm])

        glDeleteBuffers(1, [self.ebo])

# =========================================================
# Sphere
# =========================================================

class SmoothSphere:

    def __init__(self, interner):

        self.interner = interner

        self.cx = 16
        self.cy = 16
        self.cz = 16

        self.r = 8.0

        self.target_r = 8.0

        self.speed = 3

        self.grow = True

        self.min_r = 5
        self.max_r = 13

        self.dirty = False

    def tick(self, dt):

        if self.grow:

            self.target_r += self.speed * dt

            if self.target_r >= self.max_r:

                self.target_r = self.max_r
                self.grow = False

        else:

            self.target_r -= self.speed * dt

            if self.target_r <= self.min_r:

                self.target_r = self.min_r
                self.grow = True

        diff = self.target_r - self.r

        if abs(diff) > 0.05:

            self.r += diff * 0.15

            self.dirty = True

        return self.dirty

    def build_mesh(self):

        chunk = pyvoxelis.world.VoxChunk.with_position(
            1.0,
            5,
            0,0,0
        )

        chunk.generate_test_sphere(
            self.interner,
            self.cx,
            self.cy,
            self.cz,
            int(round(self.r)),
            1
        )

        return chunk.generate_mesh(
            self.interner,
            0,0,0,0
        )

# =========================================================
# Main
# =========================================================

def main():

    pygame.init()

    pygame.display.set_mode(
        (1280,720),
        pygame.DOUBLEBUF | pygame.OPENGL
    )
    pygame.mouse.set_visible(False)
    pygame.event.set_grab(True)
    pygame.display.set_caption("Dynamic Sphere")

    glEnable(GL_DEPTH_TEST)
    glEnable(GL_CULL_FACE)

    glClearColor(0.07,0.09,0.12,1)

    shader = compileProgram(
        compileShader(VERTEX_SHADER, GL_VERTEX_SHADER),
        compileShader(FRAGMENT_SHADER, GL_FRAGMENT_SHADER)
    )

    interner = pyvoxelis.interner.VoxInternerI32.with_memory_budget(
        100 * 1024 * 1024
    )

    sphere = SmoothSphere(interner)

    mesh = DynamicMesh()

    vs, ns, ids = sphere.build_mesh()

    mesh.update(vs, ns, ids)

    cam = Camera()

    clock = pygame.time.Clock()

    cooldown = 0

    running = True

    while running:

        dt = clock.tick(144) / 1000

        cooldown += dt

        for e in pygame.event.get():

            if e.type == pygame.QUIT:
                running = False

            if e.type == pygame.KEYDOWN:

                if e.key == pygame.K_ESCAPE:
                    running = False

        # ==========================================
        # sphere update
        # ==========================================

        if sphere.tick(dt) and cooldown > 0.25:

            cooldown = 0

            vs, ns, ids = sphere.build_mesh()

            mesh.update(vs, ns, ids)

            sphere.dirty = False

        # ==========================================
        # camera
        # ==========================================

        mx,my = pygame.mouse.get_pos()

        dx = mx - 640
        dy = my - 360

        pygame.mouse.set_pos((640,360))

        cam.yaw += dx * 0.12
        cam.pitch -= dy * 0.12

        cam.pitch = max(-89, min(89, cam.pitch))

        cam.update()

        keys = pygame.key.get_pressed()

        spd = cam.speed * dt

        if keys[pygame.K_LSHIFT]:
            spd *= 2

        if keys[pygame.K_w]:
            cam.pos += cam.front * spd

        if keys[pygame.K_s]:
            cam.pos -= cam.front * spd

        if keys[pygame.K_a]:
            cam.pos -= cam.right * spd

        if keys[pygame.K_d]:
            cam.pos += cam.right * spd

        if keys[pygame.K_SPACE]:
            cam.pos.y += spd

        if keys[pygame.K_LCTRL]:
            cam.pos.y -= spd

        # ==========================================
        # render
        # ==========================================

        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)

        glUseProgram(shader)

        p = glm.perspective(
            glm.radians(60),
            1280/720,
            0.1,
            200
        )

        v = cam.view()

        m = glm.mat4(1)

        glUniformMatrix4fv(
            glGetUniformLocation(shader,"proj"),
            1,
            GL_FALSE,
            glm.value_ptr(p)
        )

        glUniformMatrix4fv(
            glGetUniformLocation(shader,"view"),
            1,
            GL_FALSE,
            glm.value_ptr(v)
        )

        glUniformMatrix4fv(
            glGetUniformLocation(shader,"model"),
            1,
            GL_FALSE,
            glm.value_ptr(m)
        )

        mesh.draw()

        pygame.display.flip()

    mesh.destroy()

    pygame.quit()

if __name__ == "__main__":
    main()