(function () {
  "use strict";

  var storageKey = "rusty-weather-course-completed-v1";

  function readCompleted() {
    try {
      var value = JSON.parse(window.localStorage.getItem(storageKey) || "[]");
      return Array.isArray(value) ? value : [];
    } catch (_) {
      return [];
    }
  }

  function writeCompleted(values) {
    try {
      window.localStorage.setItem(storageKey, JSON.stringify(values));
      return true;
    } catch (_) {
      return false;
    }
  }

  document.querySelectorAll("[data-quiz]").forEach(function (quiz) {
    var options = Array.prototype.slice.call(quiz.querySelectorAll("[data-quiz-option]"));
    var output = quiz.querySelector("[data-quiz-feedback]");
    var check = quiz.querySelector("[data-check-quiz]");
    var selected = null;

    options.forEach(function (option) {
      option.setAttribute("aria-pressed", "false");
      option.addEventListener("click", function () {
        selected = option;
        options.forEach(function (item) {
          item.setAttribute("aria-pressed", item === option ? "true" : "false");
        });
        if (output) {
          output.hidden = true;
          output.removeAttribute("data-state");
        }
      });
    });

    if (check) {
      check.addEventListener("click", function () {
        if (!output) return;
        if (!selected) {
          output.textContent = "Choose one answer first, then check it.";
          output.hidden = false;
          output.setAttribute("data-state", "wrong");
          return;
        }
        var correct = selected.getAttribute("data-correct") === "true";
        output.textContent = correct
          ? quiz.getAttribute("data-feedback-correct")
          : quiz.getAttribute("data-feedback-wrong");
        output.hidden = false;
        output.setAttribute("data-state", correct ? "correct" : "wrong");
      });
    }
  });

  var lessonId = document.body.getAttribute("data-lesson-id");
  var completed = readCompleted();
  var completeButton = document.querySelector("[data-complete]");

  if (completeButton && lessonId) {
    if (completed.indexOf(lessonId) !== -1) {
      completeButton.textContent = "Lesson marked complete";
      completeButton.setAttribute("aria-pressed", "true");
    }
    completeButton.addEventListener("click", function () {
      completed = readCompleted();
      if (completed.indexOf(lessonId) === -1) completed.push(lessonId);
      var saved = writeCompleted(completed);
      completeButton.textContent = saved ? "Lesson marked complete" : "Complete for this visit";
      completeButton.setAttribute("aria-pressed", "true");
    });
  }

  var lessonLinks = document.querySelectorAll("[data-lesson-link]");
  var progressText = document.querySelector("[data-progress-text]");
  var progressFill = document.querySelector("[data-progress-fill]");
  if (lessonLinks.length && progressText && progressFill) {
    var done = readCompleted();
    var count = 0;
    lessonLinks.forEach(function (link) {
      var id = link.getAttribute("data-lesson-link");
      var complete = done.indexOf(id) !== -1;
      if (complete) count += 1;
      var badge = link.querySelector("[data-status]");
      if (badge) badge.textContent = complete ? "Done" : "Next";
    });
    progressText.textContent = count + " of " + lessonLinks.length + " lessons marked complete";
    progressFill.style.width = Math.round((count / lessonLinks.length) * 100) + "%";
    progressFill.setAttribute("aria-valuenow", String(count));
  }
}());
