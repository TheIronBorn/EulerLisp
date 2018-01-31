; Solved 22.1.18

(defn cube (x) (fx* x (fx* x x)))

(defn count-other_ (cur target (cnt 0))
  (let ([next (~> cur cube number->digits sort)])
    (cond
      [(> (length next) (length target)) cnt]
      [(= next target) (count-other_ (inc cur) target (inc cnt))]
      [else (count-other_ (inc cur) target cnt)])))

(defn count-other (n)
  (println "n = " n)
  (count-other_ (inc n) (sort (number->digits (cube n)))))

(println (count-other 345))

(~>
  (step~ 1)
  (select~ &(= (count-other &1) 4))
  first~
  cube
  (println "Solution: "))
