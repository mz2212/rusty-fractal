uniform dvec2 loc;
uniform dvec2 julia_data;
uniform double zoom;
uniform int max_iter;
uniform uint width;
uniform uint height;
out vec4 color;

void main() {
	dvec2 old, new;
	vec3 hsv;
	
	new.x = (1.5*(double(gl_FragCoord.x)-double(width)/2.0)/(0.5*zoom*double(width) + loc.x));
	new.y = (double(gl_FragCoord.y) - double(height) / 2.0)/(0.5*zoom*double(height) + loc.y);

	int i;
	for (i = 0; i < max_iter; i++) {
		old = new;

		new.x = old.x * old.x - old.y * old.y + julia_data.x;
		new.y = 2 * old.x * old.y + julia_data.y;
		if ((new.x * new.x + new.y * new.y) > 4.0) {
			break;
		}
	}

	hsv[0] = float(i) / float(max_iter);
	hsv[1] = 1.0;
	hsv[2] = float(i < max_iter) * 1.0;
	color = vec4(hsv2rgb(hsv), 1.0);
}