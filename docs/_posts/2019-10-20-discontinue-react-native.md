# Discontinue react-native

For some time, graffiti had an experimental react-native renderer/reconciler (with a subset of RN api) which means you could theoretically write RN application and run it not only on all standard RN platforms but also on raspberry pi and other graffiti-supported devices.

Or at least that was the idea, or to be 100% honest it was more like that I was not sure what exactly I want to support and so I thought I'll just start with something like react-native and see how far it would get me.

Good thing about RN is that you can use the same React, hooks, react-devtools, some of the libraries (more on that later) and there's also flexbox layout (everything is `display: flex` in RN) and very simple API - basically it's just `<View` & `<Text` all the way down.

The bad thing is that there's no DOM and a lot of libraries depend on that so you're limited only to the most popular ones where somebody already reimplemented the whole thing for RN (e.g. `styled-components`). What's even worse is that flexbox & the styles in overall are not just subset of native styles, they are intentionally different (`flexDirection: column` is default for example) so it's not like you could support both easily. Another pain point is that there's no `<input` and even events are named differently. And to make things even worse, you couldn't use some of react-native libraries simply because they often have some native code specific for given platform (iOS/Android).

In short, I was not very happy with the situation so something had to be done but I didn't know what for quite a long time.

Apart from the RN part, there was also pseudo-DOM part, which I've spinned off just to see if I could support vue/angular & some other frameworks simply by providing enough subset of DOM to bridge it to the low-level parts which were interfaced directly by the custom RN reconciler. It was just PoC but it made clear that this is a viable way and despite being a bit less efficient it had some advantages in terms of libraries.

So maybe that something what had to be done could be the pseudo-DOM. So I went into expanding that idea a bit more and now you can use a lot of existing libs, for example `react-spatial-navigation`, `react-window` or `wouter` and probably a lot of others too. But what's really cool is that it should only get better in the future so it really feels like the better option, given the situation in which I am.

That leads me back to the RN part, which had one other notable (subjective) downside - I am not actively involved in the RN community so it's **very hard** for me to keep up with the community, guess where they are heading, what is their philosophy, architecture decisions, etc. Also, docs are often obsolete, source code is changing a lot, TS types too, so I made a decision to remove the RN part completely and focus more on the psuedo-DOM part which should theoretically be better for everybody.

Now, it might sound really weird - I've removed something which was already working (to some degree) and that is a bad thing of course but the sad truth is that nobody was using it anyway and probably, nobody even wanted that in the first place. People give you stars and say it's good and interesting but nobody is going to use it anyway because it's so different from real web and then they could just use react-native & not waste their time with some half-working alternative. And it was big win for maintenance too because react-renconciler is private API which is changing a lot and it's really hard to keep up with that and I can't even imagine having to support many different versions if somebody actually used it already.

So the new direction is to get a bit more closer to the web, make it possible to use most of the react & javascript libraries you're used to at the browser and see how that will work out. This includes not only react, preact, vue, mithril or even jquery but also most of the plugins & libraries for those frameworks.

The only serious limitation (excluding the bugs & TODOs) is the lack of global stylesheets & selector rules. It's not that it would be that hard to add it but it's very interesting how far I could get with just inline styles so I'd like to keep it that way until it will really hurt a lot, if that wil ever happen.

And if you're aware of what's happening in the design system community, one of the interesting trends is that components are self-contained, independent from global rules so that they can be used anywhere and they will always look the same. So you can't use bootstrap with graffiti because bootstrap needs global styles but you could totally do your own components with inline styles only and it could look exactly the same.
