# more-blue

Another very tiny codec? Make your (y4m-structured datastream formatted) videos MORE BLUE!

`cargo build` and  

`cargo run data/example.y4m data/output.y4m`  

Make another raw data stream to test:   

`ffmpeg -f lavfi -i mandelbrot=size=1280x720:rate=25 -pix_fmt yuv420p -t 2 test.y4m`  

View results with `ffplay`:  

`ffplay data/output.y4m`  

Mux data stream into video wrapper:  

`ffmpeg -i test.y4m -vcodec copy test.mkv`  