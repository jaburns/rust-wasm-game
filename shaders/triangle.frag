precision highp float;

uniform float u_time;

void main()
{
	gl_FragColor = vec4(vec2(.7,.7) + vec2(.2,.2)*sin(u_time/100.), 0,1);
}