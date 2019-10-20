# New name

Looking at the date of the previous posts, it's been a long time :-)
I've started writing this when I was waiting at the airport for my next flight
but I didn't have enough time to finish it and basically the whole summer
was this way - time was limited and if I had a chance, I was rather coding
than writing about it.

So quickly to the status of the project - it's still one-man show,
I'm still working on it, it's far from being dead but the time is limited
and sometimes it's just not possible to squeeze few extra hours at night to
hack on the next features (this time I was going through USA national parks so
a lot of walking, lot of driving, very little of sleeping and almost no coding)

Continuing from the last post, I've decided to drop webrender and
go with my own custom opengl renderer which of course turned out to be
a bit more difficult than I expected but that was rather because I was
a total novice in that area - it is actually very simple now when I'm looking
at it and it's done already. I also still think it was good idea to go with
custom solution instead of using skia for example as it would be another C++
dependency with another build system and it's complex enough already.
And skia is huge, when compared to what we have now.

Text-rendering is now almost finished, there are some minor bugs, some features
are missing, but it's good enough so that I'm not ashamed to show it to
my friends. I'm currently working on releasing the very first stable version,
not that it would be feature complete but the codebase and architecture is now
good enough so that I think it can survive another year without any significant
rewrites.

It's still not a real competitor to react-native, nor electron but it should
work fine on raspberry pi and with let's say virtual keyboard it could be
interesting for some embedded applications. I mean, you can now do UI apps
in javascript and it will run fluently on raspi 3. I don't know about any other
solution being capable of doing that.

Related to a first stable release I was also thinking a lot about a new name,
there were some good ideas from other people but I've decided to name it
**graffiti**, not that it's something smart or nice but because it's very
easy to remember, or at least that is what I hope and because I like it the most
from all of the other options I was considering.

There are no ETAs, the last one was overdue by a few months so I'm even
less happy to give any more dates but I'm very optimistic about the project now
and I'm sure I will be able to give myself a nice christmas gift, hopefully
the early one :)
