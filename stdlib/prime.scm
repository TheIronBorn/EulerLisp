(defn add_factor (factor card acc)
  (if (= card 0)
    acc
    (cons (cons factor card) acc)))

(defn prime_factors_ (n cur card acc)
  (if (> cur n)
      (add_factor cur card acc)
      (if (divides? cur n)
          (prime_factors_ (/ n cur) cur (inc card) acc)
          (prime_factors_ n (inc cur) 0 (add_factor cur card acc)))))

(defn prime_factors (n)
  (prime_factors_ n 2 0 '()))
