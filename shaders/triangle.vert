attribute vec4 a_position;
uniform float u_time;

void main()
{
	vec4 pos = a_position;
	pos.y += 0.25;
	pos.xy *= mat2(cos(u_time/971.), sin(u_time/971.), -sin(u_time/971.), cos(u_time/971.));
	gl_Position = pos;
}