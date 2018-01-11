(defn pentagonal (n)
      (/ (* n (- (* 3 n) 1)) 2))

(defn pentagonal? (n cur)
   (let (pcur (pentagonal cur))
     (cond
       (< pcur n) (pentagonal? n (inc cur))
       (= pcur n) #t
       else #f)))

(defn find-first (a b)
      (println a)
      (if (>= b a)
        (find-first (inc a) 1)
        (let (pa (pentagonal a)
              pb (pentagonal b))
          (if (and
                (pentagonal? (+ pa pb) 1)
                (pentagonal? (- pa pb) 1))
            (println (cons a b))
            (find-first a (inc b))))))

(find-first 2 1)
