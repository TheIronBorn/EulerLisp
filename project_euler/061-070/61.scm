(defn triangle   (n) (div (* n (+ n 1)) 2))
(defn square_    (n) (* n n)) 
(defn pentagonal (n) (div (* n (- (* 3 n) 1)) 2))
(defn hexagonal  (n) (* n (- (* 2 n) 1))) 
(defn heptagonal (n) (div (* n (- (* 5 n) 3)) 2))
(defn octagonal  (n) (* n (- (* 3 n) 2))) 

(defn valid? (n) (not (zero? (% (div n 10) 10))))

(defn find-four-digit (f (n 1) (acc '()))
  (let (next (f n))
    (cond
      (< next 1000) (find-four-digit f (inc n) acc)
      (> next 9999)
        (~>
          acc
          (select valid?)
          (map &(divmod &1 100)))
      else (find-four-digit f (inc n) (cons next acc)))))

; Start w/ octagonal because that has the fewest elements
(def numbers
     (map find-four-digit
          (list octagonal heptagonal hexagonal pentagonal square_ triangle)))

(defn find-sequences (start end lists (acc '()))
  (if (= 1 (length lists))
      (map list
        (select
          &(= &1 (cons start end))
          (fst lists)))
      (flatmap
        (fn (x)
            (let (new-start (rst x))
              (map &(cons x &1)
                (find-sequences new-start end (rst lists)))))
        (select
          &(= (fst &1) start)
          (fst lists)))))

(defn sequences (first rest)
     (flatmap
       (fn (x)
         (map
           &(cons x &1)
           (find-sequences (rst x) (fst x) rest)))
       first))

(~>
  (rst numbers)
  permutations
  (flatmap &(sequences (fst numbers) &1))
  fst
  println-id
  (map &(+ (* 100 (fst &1)) (rst &1)))
  sum
  (println "Solution: "))
