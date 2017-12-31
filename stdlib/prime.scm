(defn prime-factors (n)
  (defn add-factor (factor card acc)
    (if (= card 0)
      acc
      (cons (cons factor card) acc)))

  (defn helper (n cur card acc)
    (if (> cur n)
        (add-factor cur card acc)
        (if (divides? cur n)
            (helper (/ n cur) cur (inc card) acc)
            (helper n (inc cur) 0 (add-factor cur card acc)))))
  (helper n 2 0 '()))
