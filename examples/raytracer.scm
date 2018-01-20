; Times:
;  * 19.1.18 18.24s
;  * 20.1.18 13.38s, switch to activation frames

(defn make-vec (x y z)
      (list x y z))

(def vec-x fst)
(def vec-y frst)
(def vec-z frrst)

(defn vec+ (va vb)
      (make-vec (+ (vec-x va) (vec-x vb))
                (+ (vec-y va) (vec-y vb))
                (+ (vec-z va) (vec-z vb))))

(defn vec- (va vb)
      (make-vec (- (vec-x va) (vec-x vb))
                (- (vec-y va) (vec-y vb))
                (- (vec-z va) (vec-z vb))))

(defn vec* (va s)
      (make-vec (* (vec-x va) s)
                (* (vec-y va) s)
                (* (vec-z va) s)))

(defn vec-dot (va vb)
      (+ (* (vec-x va) (vec-x vb))
         (* (vec-y va) (vec-y vb))
         (* (vec-z va) (vec-z vb))))

(defn make-ray (o d) (cons o d))
(def ray-origin fst)
(def ray-direction rst)

(defn make-sphere ()
      (fn (ray)
          (let (d (ray-direction ray)
                o (ray-origin ray))
            (let (a (vec-dot d d)
                  b (* 2 (vec-dot d o))
                  c (- (vec-dot o o) 1))
              (let (discriminant (- (* b b) (* 4 a c)))
                (if (>= discriminant 0)
                  (let (t1 (/ (+ (- b) (sqrt discriminant)) (* 2 a))
                        t2 (/ (- (- b) (sqrt discriminant)) (* 2 a)))
                    (list
                      (min t1 t2)
                      (vec+ o (vec* d (min t1 t2)))))))))))

(def s1 (make-sphere))

(defn loop (obj (x 0) (y 0))
  (cond
    (>= y 400) 'done
    (>= x 800) (loop obj 0 (inc y))
    else
    (do
      (let (x_ (->float (/ (- x 400) 100))
            y_ (->float (/ (- y 200) 100)))
        (let (hit (obj
                    (make-ray (make-vec x_ y_ 10)
                              (make-vec 0 0 -1))))
          (if (nil? hit)
              (println "0 0 0")
              (println "255 255 255"))
          (loop obj (inc x) y)
      )
    ))))

(println "P3 800 400 255")
(loop s1)
