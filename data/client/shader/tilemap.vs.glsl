

#version 130

//uniform mat4 mvp;	//model view projection matrix
					//model -> hold model position
					//view -> camera
					//projection -> 3d world(view) to 2d screen

uniform mat4 view;
uniform mat4 projm;

in vec2 vertex;		//input of the vertexes to render

void main()
{
	gl_Position = view * vec4(vertex, 0.0, 1.0);
}
