; Solved 18.1

(defn ring-numbers (i)
  (let (sq (square (inc (* 2 i))))
    (list
      sq
      (- sq (* i 2))
      (- sq (* i 4))
      (- sq (* i 6)))))

(defn solve ((i 2) (numbers 5) (ps 3))
    (if (< (/ ps numbers) (/ 1 10))
        (inc (* 2 (dec i)))
        (solve (inc i)
               (+ numbers 4)
               (+ ps (count prime? (ring-numbers i))))))

(println "Solution: " (solve))
