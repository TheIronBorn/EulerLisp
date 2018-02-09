; Solved 9.1.2018

(def ranges (map &(* &1 9 (pow 10 (dec &1))) (range 1 7)))

(defn find-bracket_ (n cur ranges)
  (if (< n (fst ranges))
      (list cur n)
      (find-bracket_ (- n (fst ranges)) (inc cur) (rst ranges))))
(defn find-bracket (n)
      (find-bracket_ n 1 ranges))

(defn find-digit (range rest)
    (let* ([real-rest (+ rest (* range (pow 10 (dec range))))]
           [position (dec (- range (% real-rest range)))])
         (% (div (div real-rest range) (pow 10 position)) 10)))

(defn digit (n)
  (if (<= n 9)
      n
      (~> n dec find-bracket (apply find-digit))))

(solution (reduce-product &(digit (pow 10 &1)) (range 0 6)))
