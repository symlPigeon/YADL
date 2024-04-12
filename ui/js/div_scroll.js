let animation;

function startScrolling(paragraph) {
    const paragraphWidth = paragraph.scrollWidth;
    const containerWidth = paragraph.clientWidth;

    if (paragraphWidth > containerWidth) {
        const duration = ((paragraphWidth - containerWidth) / 50) * 1000; // 根据滚动距离计算滚动时间
        console.log(duration);
        scroll_percent = 100 - (containerWidth / paragraphWidth) * 100 + 5;
        document.documentElement.style.setProperty(
            "--max_scroll_percent",
            `-${scroll_percent}%`
        );

        paragraph.style.animationDuration = `${duration}ms`;
        paragraph.style.animationIterationCount = "1";

        animation = setTimeout(function () {
            resetScrolling(paragraph);
        }, duration + 1000);
    }
}

function resetScrolling(paragraph) {
    paragraph.style.animation = "none";
    paragraph.offsetHeight; // trigger reflow
    paragraph.style.animation = null;
    animation = requestAnimationFrame(() => {
        startScrolling(paragraph);
    });
}
