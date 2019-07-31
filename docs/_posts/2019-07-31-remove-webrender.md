# Moving away from webrender

When this project started, it was **solely** about [webrender](https://github.com/servo/webrender),
I was reading about it a lot so I simply wanted to give it a try
and even try rust along the way, because to get an idea about any language/lib/framework you really need to use it for some non-trivial task.

The idea was simple, let webrender handle rendering, use yoga for the layout and make some kind of bridge so that it can be used from javascript, or specifically from the node.js.

Of course, it was not that simple but I eventually got it to the point I had essentials working and the project started to get a bit more serious.

Over time I've started doubting a bit about webrender. I mean, I really, really appreciate what Mozilla is doing and I believe that it's also superior renderer for a **web browser**. But for a GUI toolkit, webrender is doing a lot of extra work which is not necessary and sometimes it doesn't even make sense.

# Different constraints
For example, in a browser, you have a lot of tabs (or even iframes) and those can be async updated from javascript at any time. So that's probably why to render anything you need to first make a request, serialize it and send it to the webrender which will then deserialize it again and do the necessary work. This, of course, has some perf penalty which is still a good price for having a nice thread-safe design which is important because browsers are running 3rd party (untrusted) code on your computer.

Another thing is that since that you typically have a lot of webpages open at the same time, it makes sense to do some heavy caching which again is great for browser but not so great for a native app.

Not to mention this overhead would be even more noticeable on embedded devices.

## Pathfinder
Another very interesting lib in the rust world is [pathfinder](https://github.com/servo/pathfinder). I heard about it because it was eventually supposed to replace freetype for glyph rendering in the webrender. Over the time the project got a bit more ambitious and now it's able to draw simple SVGs and it's becoming to be a bit like what [skia](https://github.com/google/skia) is to the c++ world.

Nice thing is that since pathfinder is a bit simpler it's also easier for me to use it. OSS projects usually lack documentation so it's often about looking into the source code to find out how something works and try to guess how it is supposed to be used. With webrender it was trial & error in a lot of cases. After more than 1 year of using it there's still a lot of about it which I don't know.

So I initially thought I could use it instead of webrender, as a lightweight renderer and just implement what's missing but it turns out it's not so easy because I would need to solve:

- clipping (div with border-radius and picture in it)
- hit-testing (mouse events)
- scrolling
- shadows (& round box shadows)

And what's worse, it's not easy (if possible at all) to render something custom in the middle of the rasterization process so this is also probably a dead-end.

# Custom renderer
I've recently come across a project named [makepad](https://github.com/makepad/makepad) which is also doing the whole UI on the GPU and although it's very interesting, it's also totally incompatible with CSS way of defining appearances.

The reason why I'm mentioning it is that the author of this project made a lot of things himself, instead of relying on existing packages. And this was really inspiring for me because for some reason I was afraid of getting my hands dirty with GPU but it turns out that if you can control the whole stack, a lot of things get much much easier.

For example, to fill some opaque rectangles, you need 4 vertices and 2 triangles and these vertices are not going to change unless there was some change in layout, which you can know in advance (layout property was touched).

Simply put, I've started experimenting with GL and so far it looks very promising. There are things which are going to be challenging but again, I have much much simpler scope than a web browser, I can also afford to not support certain features which would make rendering way slower.

I still have to solve a lot of things and it's going to take a while but I believe it's worth because I can:

- support WebGL (for the online playground)
- support Raspberry PI
- keep the memory really low
- keep the code a bit simpler (abstractions have its cost)
- avoid silly bugs because I don't understand how some lib works internally
