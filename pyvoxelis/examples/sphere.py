import pygame
import numpy as np
from OpenGL.GL import *
from OpenGL.GLU import *
from OpenGL.GL.shaders import compileProgram, compileShader

import pyvoxelis

# ==============================================
# 着色器
# ==============================================
vertex_shader = """
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

out vec3 normal;

void main() {
    gl_Position = proj * view * model * vec4(aPos, 1.0);
    normal = aNormal;
}
"""

fragment_shader = """
#version 330 core
in vec3 normal;
out vec4 FragColor;

void main() {
    vec3 n = normalize(normal);
    vec3 color;

    if (n.y > 0.5)
        color = vec3(0.95, 0.85, 0.65);
    else if (n.y < -0.5)
        color = vec3(0.5, 0.4, 0.3);
    else if (abs(n.z) > 0.5)
        color = vec3(0.8, 0.7, 0.5);
    else if (abs(n.x) > 0.5)
        color = vec3(0.7, 0.6, 0.45);
    else
        color = vec3(0.85, 0.75, 0.55);

    FragColor = vec4(color, 1.0);
}
"""

# ==============================================
# 初始化
# ==============================================
pygame.init()
screen = pygame.display.set_mode((1280, 720), pygame.DOUBLEBUF | pygame.OPENGL)
pygame.display.set_caption("Voxelis 3D sphere")

glEnable(GL_DEPTH_TEST)
glEnable(GL_CULL_FACE)
glCullFace(GL_BACK)   # 剔除背面
glClearColor(0.08, 0.1, 0.16, 1)



# 编译着色器
shader = compileProgram(
    compileShader(vertex_shader, GL_VERTEX_SHADER),
    compileShader(fragment_shader, GL_FRAGMENT_SHADER)
)

# ==============================================
# 体素数据
# ==============================================
interner = pyvoxelis.interner.VoxInternerI32.with_memory_budget(1024 * 1024 * 100)
chunk = pyvoxelis.world.VoxChunk.with_position(1.0, 5, 0, 0, 0)
chunk.generate_test_sphere(interner, 16, 16, 16, 14, 1)
vertices, normals, indices = chunk.generate_mesh(interner, 0, 0.0, 0.0, 0.0)

# ==============================================
# 上传到GPU
# ==============================================
vertices = np.array(vertices, dtype=np.float32)
normals = np.array(normals, dtype=np.float32)
indices = np.array(indices, dtype=np.uint32)

VAO = glGenVertexArrays(1)
VBO = glGenBuffers(2)
EBO = glGenBuffers(1)

glBindVertexArray(VAO)
glBindBuffer(GL_ARRAY_BUFFER, VBO[0])
glBufferData(GL_ARRAY_BUFFER, vertices.nbytes, vertices, GL_STATIC_DRAW)
glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 0, None)
glEnableVertexAttribArray(0)

glBindBuffer(GL_ARRAY_BUFFER, VBO[1])
glBufferData(GL_ARRAY_BUFFER, normals.nbytes, normals, GL_STATIC_DRAW)
glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, 0, None)
glEnableVertexAttribArray(1)

glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, EBO)
glBufferData(GL_ELEMENT_ARRAY_BUFFER, indices.nbytes, indices, GL_STATIC_DRAW)
glBindVertexArray(0)

# ==============================================
# 渲染循环
# ==============================================
angle_x = 0
angle_y = 0
clock = pygame.time.Clock()
running = True

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT or (event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE):
            running = False

    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
    glUseProgram(shader)

    # --------------------------
    # 投影矩阵
    # --------------------------
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    gluPerspective(60, 1280/720, 0.1, 100)
    proj_mat = glGetFloatv(GL_PROJECTION_MATRIX)

    # --------------------------
    # 视图矩阵（相机）
    # --------------------------
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()
    gluLookAt(1, 1.5, 0.5,  0,0,0,  0,1,0)
    view_mat = glGetFloatv(GL_MODELVIEW_MATRIX)

    # --------------------------
    # 模型矩阵（旋转）
    # --------------------------
    glLoadIdentity()
    glRotatef(angle_y, 1,0,0)
    glRotatef(angle_x, 0,1,0)
    model_mat = glGetFloatv(GL_MODELVIEW_MATRIX)

    # --------------------------
    # 传给着色器
    # --------------------------
    glUniformMatrix4fv(glGetUniformLocation(shader, "proj"), 1, GL_FALSE, proj_mat)
    glUniformMatrix4fv(glGetUniformLocation(shader, "view"), 1, GL_FALSE, view_mat)
    glUniformMatrix4fv(glGetUniformLocation(shader, "model"), 1, GL_FALSE, model_mat)

    # --------------------------
    # 渲染
    # --------------------------
    glBindVertexArray(VAO)
    glDrawElements(GL_TRIANGLES, len(indices), GL_UNSIGNED_INT, None)

    pygame.display.flip()

    angle_x += 0.3
    angle_y += 0.15
    clock.tick(100)

pygame.quit()