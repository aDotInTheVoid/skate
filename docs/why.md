# Why Skate

<div class="warning">

Warning: Today Skate only exists as ideas. This document is nothing but planing, and
their is nothing you can use today. However, if you believe the ideas presented
here are worth working towards, I'd love to have you help.

</div>

> A computer has always been a bycicle of the mind.

— [Steve Jobs](https://youtu.be/KmuP8gsgWb8?t=51)

Do you ever watch those velodrome events in the Olympics? They either have a rolling start, or they have to have someone holding them up for a standing start. The bikers look silly and helpless until they get moving. And then they go in a loop, round and round, very, very fast. If you turn a little too hard your wheels slide out from under you and you eat it. When bikers crash in the velodrome the road rash is horriffic and their paper-thin costumes are torn the shreds.

What if we thought of the computer as a "skateboard for the mind?"

Skateboarding is improvisation. Repeated failure. Creative recombination. There's no "right way" to skate. It's also much slower than riding a bike. But much cooler.

The tradeoffs we've made in the development of programing languages since the 60s have favored the "efficiency" and "raw speed" of the velodrome over the ability to turn your own way.



The greatest bicycle I've ever used is Go. It's fast, responcive and incredably productive. And yet, it can only do what the go team at 
google allow it to do. Trying to build a lazy iterator in Go like riding the wrong way in velodrome. "That's not what it's for!". Building
your own primitaves in Go is litteraly impossable, unless you just mean using `interface{}` and type assertions like a jokester.


What would a "skateboard" language be like? How do we get there?

<style>
main .warning p {
    padding: 10px 20px;
    margin: 20px 0;
}

main .warning p::before {
    content: "⚠️ ";
}

.light main .warning p,
.rust main .warning p {
    border: 2px solid red;
    background: #ffcece;
}

.rust main .warning p {
    /* overrides previous declaration */
    border-color: #961717;
}

.coal main .warning p,
.navy main .warning p,
.ayu main .warning p {
    background: #542626
}

/* Make the links higher contrast on dark themes */
.coal main .warning p a,
.navy main .warning p a,
.ayu main .warning p a {
    color: #80d0d0
}
</style>