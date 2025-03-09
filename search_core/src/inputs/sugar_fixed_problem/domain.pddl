; Domain Overview and Improvements
; This domain models an industrial supply chain for sugar production, where raw cane is processed into refined sugar.
; It includes various stages such as production at mills, resource acquisition from farms or other mills, and
; transportation using trucks and cranes. The domain employs numeric functions and logical predicates to manage
; resource flows, operational states, and associated costs across the supply chain.
;
; I'm fixing the current domain based on the reference paper below and making changes to improve its optimality
; and clarity.
;
; Reference Paper:
; Coles, Amanda, M. Fox, and D. Long.
; "A hybrid LP-RPG heuristic for modelling numeric resource flows in planning."
; Journal of Artificial Intelligence Research 46 (2013): 343-412.
;
; Author of the New Domain:
; DavidIzhaki
; Email: davidizhakinew@gmail.com

(define (domain supply-chain)
	(:requirements :typing :fluents :equality)
	(:types 
        ; brand: different types or varieties of sugar.
        ; raw-cane: the unrefined material used to produce sugar.
        ; mill: production facility that processes raw cane into sugar.
        ; depot: storage or distribution facility.
        ; farm-field: field on a farm for harvesting raw cane.
        ; truck: vehicle used for transporting sugar.
        ; crane: mechanical loader used for handling sugar.

		 sugar location loader - object
		 brand raw-cane          - sugar
		 mill depot farm-field - location
		 truck crane	       - loader	
	)
    
	(:predicates
        ; Indicates that mill ?m is available for production
		(available ?m - mill)

        ; Specifies that mill ?m is capable of producing sugar of brand ?b.
		(can-produce ?m - mill ?b - brand)

        ; This predicate indicates that a particular brand (?b) of sugar is produced using a single type of raw cane (?r).
        (single-raw-cane-production ?r - raw-cane ?b - brand)

        ; This predicate indicates that a particular brand (?b) of sugar is produced by mixing two types of raw cane (?r1 and ?r2).
        (mixed-raw-cane-production ?r1 ?r2 - raw-cane ?b - brand)

        ; Denotes the current production process at mill ?m producing sugar of brand ?b.
		(current-brand-process ?m - mill ?b - brand)

        ; Allows the change of production process from brand ?b1 to brand ?b2.
		(change-brand-process ?b1 ?b2 - brand)

        ; Represents that mill ?m has placed an order for raw cane ?r.
		(place-order ?r - raw-cane ?m - mill)

        ; Indicates that the loader (truck, crane, etc.) ?d is at location ?l.
		(at-location ?d - loader  ?l - location)

        ; Indicates that the farm-field ?f is located near or is associated with mill ?m.
        (farmfield-near-mill ?f -farm-field ?m -milis)

        ; Indicates that raw cane of type ?r can be harvested from the specified farm field ?f.
        (farmfield-harvest-cane ?f -farm-field ?r -raw-cane)

        ; Specifies that location ?l1 is connected to location ?l2, enabling movement between them.
		(connected ?l1 ?l2 - location)

        ; Indicates that mill ?m1 is connected to mill ?m2.
        (connected-mills ?m1 ?m2 -mill)
	)

    	(:functions

        ; Represents the cumulative cost incurred by all mill operations across the supply chain.
        (mills-total-cost)
        
        ; Represents the cost associated with running the current production process at a specific mill ?m.
        (process-cost ?m - mill)

        ; Represents the maximum amount of sugar that mill ?m can produce in a single production batch.
        (max-produce ?m - mill)
	
        ; Represents the amount of cane that can be harvested per harvest action from a specific farm-field.
        (cane-yield ?r -raw-cane ?f - farm-field)
        
        ; Indicates the total amount of cane currently available on a specific farm-field.
        (total-canes ?f - farm-field)

        ; Represents the maximum amount of sugar that the crane can handle in a single operation.
        (capacity ?c - crane)
  
        ; Indicates the remaining capacity of truck ?t, i.e., the amount of sugar that can still be loaded onto it.
        (truck-remaining-capacity ?t - truck)

        ; Represents the amount of sugar of ?s currently loaded in truck ?t.
        (truck-load-sugar ?s - sugar ?t - truck)
        
        ; Represents the amount of sugar of brand ?b currently stored at location ?m.
        (in-storage ?l - location ?b - brand)
	
        ; Represents the cumulative distance traveled by vehicles in the supply chain, used for cost estimation.
        ; (total-distance) we can add this
  
        ; Specifies the distance between two locations ?l1 and ?l2, which can be used to calculate transportation costs.
        ; (distance ?l1 ?l2 - location) we can add this

        ; Tracks the quantity of raw cane available at mill ?m.
        (has-resource ?r - raw-cane ?m - mill)
        
        ; Defines the maximum number of production process changes allowed for mill ?m.
        (max-changing ?m - mill)

		; (inventory-cost) deleted we can add it. 
		
		; Represents the maintenance requirement level for crane ?c.
        ; A value of 0 means maintenance is needed.
        (need-maintenance ?c - crane)

  
        ; Represents the cumulative time spent on maintenance across all crane operations.
        (total-maintenance-time)

        ; This value indicate the time spent on maintenance a crane.
        (maintenance-time ?c - crane)
	
		; Represents the cumulative cost incurred from handling operations,
        ; such as loading and unloading sugar. This cost reflects the labor and equipment expenses involved in these processes.
        (handling-cost)
	
	)

    ; Action: produce_sugar_from_single_raw
    ; Produces one unit of sugar.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from single raw cane ?r
    ;   - At least one unit of raw cane is available.
    ; Effects:
    ;   - Increases storage by 1 unit.
    ;   - Decreases raw cane by 1 unit.
    ;   - Marks the mill as unavailable (busy) for the next production cycle.
    ;   - Increases the mill's total cost by the process cost.
    
    
    (:action produce_sugar_from_single_raw
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
                (single-raw-cane-production ?r ?b)
				(available ?m)
				(>(has-resource ?r ?m)0)
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)1)
				(decrease (has-resource ?r ?m)1)
				(not(available ?m))
				(increase (mills-total-cost)(process-cost ?m))
		     	     )
	)

    ; Action: produce_sugar_from_mixed_raw
    ; This action produces sugar of a given brand by mixing two types of raw cane.
    ; Preconditions:
    ;   - The mill’s current production process is set to produce sugar of brand ?b.
    ;   - The sugar can produced from mixed raw cane ?r1 ?r2.
    ;   - The mill is available for production.
    ;   - The mill has at least one unit of raw cane ?r1.
    ;   - The mill has at least one unit of raw cane ?r2.
    ; Effects:
    ;   - Increases the storage of sugar of brand ?b at the mill by one unit.
    ;   - Decreases the raw cane resource ?r1 by one unit.
    ;   - Decreases the raw cane resource ?r2 by one unit.
    ;   - Marks the mill as unavailable (busy) for further production until reset.
    ;   - Increases the mill's total cost by the process cost.
    
    
    (:action produce_sugar_from_mixed_raw
        :parameters (?r1 ?r2 - raw-cane ?m - mill ?b - brand)
        :precondition (and 
            (current-brand-process ?m ?b)         ; The mill's current process is set for brand ?b.
            (mixed-raw-cane-production ?r1 ?r2 ?b)   ; Indicates that brand ?b is produced by mixing raw cane types ?r1 and ?r2.
            (available ?m)                         ; The mill is available for production.
            (> (has-resource ?r1 ?m) 0)              ; There is at least one unit of raw cane ?r1.
            (> (has-resource ?r2 ?m) 0)              ; There is at least one unit of raw cane ?r2.
        )
        :effect (and
            (increase (in-storage ?m ?b) 1)          ; Increase the stored sugar of brand ?b by 1 unit.
            (decrease (has-resource ?r1 ?m) 1)         ; Decrease the resource ?r1 by 1 unit.
            (decrease (has-resource ?r2 ?m) 1)         ; Decrease the resource ?r2 by 1 unit.
            (not (available ?m))                     ; Mark the mill as unavailable after production.
            (increase (mills-total-cost) (process-cost ?m)) ; Increase the total mill cost by the process cost.
        )
    )

    ; Action: produce_sugar_from_single_raw_max
    ; Produces the maximum batch of sugar when enough raw cane is available.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from single raw cane ?r
    ;   - The available raw cane is at least equal to the maximum batch size (max-produce).
    ; Effects:
    ;   - Increases storage by max-produce units.
    ;   - Decreases raw cane by max-produce units.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 5 times the process cost.
	
    
    (:action produce_sugar_from_single_raw_max
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
                (single-raw-cane-production ?r ?b)
				(available ?m)
			    (>=(has-resource ?r ?m)(max-produce ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(max-produce ?m))
				(decrease (has-resource ?r ?m)(max-produce ?m))
				(not(available ?m))
				(increase (mills-total-cost) (* 5 (process-cost ?m)))
		     	     )
	)

    
    ; Action: produce_sugar_from_mixed_raw_max
    ; Produces the maximum batch of sugar when enough raw cane is available.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from mixed raw cane ?r1 ?r2
    ;   - The available raw cane is at least equal to the maximum batch size (max-produce).
    ; Effects:
    ;   - Increases storage by max-produce units.
    ;   - Decreases raw cane by max-produce units.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 5 times the process cost.
	
    
    (:action produce_sugar_from_mixed_raw_max
		:parameters (?r1 ?r2 - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
                (mixed-raw-cane-production ?r1 ?r2 ?b)
				(available ?m)
			    (>=(has-resource ?r1 ?m)(max-produce ?m))
                (>=(has-resource ?r2 ?m)(max-produce ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(max-produce ?m))
				(decrease (has-resource ?r1 ?m)(max-produce ?m))
                (decrease (has-resource ?r2 ?m)(max-produce ?m))
				(not(available ?m))
				(increase (mills-total-cost) (* 5 (process-cost ?m)))
		     	     )
	)

    ; Action: produce_sugar_from_single_raw_resource
    ; Produces sugar using all the available raw cane when it is less than the maximum batch size.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from single raw cane ?r
    ;   - The available raw cane is less than max-produce.
    ; Effects:
    ;   - Increases storage by the exact amount of raw cane available.
    ;   - Sets the raw cane resource at the mill to zero.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 4 times the process cost.
    
    
    (:action produce_sugar_from_single_raw_resource
		:parameters (?r - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
                (single-raw-cane-production ?r ?b)
			    (< (has-resource ?r ?m)(max-produce ?m))
                (> (has-resource ?r ?m)0)
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(has-resource ?r ?m))
				(assign (has-resource ?r ?m)0)
				(not(available ?m))
				(increase (mills-total-cost) (* 4 (process-cost ?m)))
		     	     )
	)

    ; Action: produce_sugar_from_mixed_raw_resource_r1_less
    ; Produces sugar using all the available raw cane when it is less than the maximum batch size.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from mixed raw cane ?r1 ?r2
    ;   - The available raw cane is less than max-produce.
    ; Effects:
    ;   - Increases storage by the exact amount of raw cane available.
    ;   - Sets the raw cane resource at the mill to zero.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 4 times the process cost.
   
   
    (:action produce_sugar_from_mixed_raw_resource
		:parameters (?r1 ?r2 - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
                (mixed-raw-cane-production ?r1 ?r2 ?b)
			    (< (has-resource ?r1 ?m)(max-produce ?m))
                (< (has-resource ?r2 ?m)(max-produce ?m))
                (> (has-resource ?r1 ?m)0)
                (> (has-resource ?r2 ?m)0)
                (< (has-resource ?r1 ?m)(has-resource ?r2 ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(has-resource ?r1 ?m))
                (decrease (has-resource ?r2 ?m)(has-resource ?r1 ?m))
				(assign (has-resource ?r1 ?m)0)
				(not(available ?m))
				(increase (mills-total-cost) (* 4 (process-cost ?m)))
		     	     )
	)
    
    ; Action: produce_sugar_from_mixed_raw_resource_r2_less
    ; Produces sugar using all the available raw cane when it is less than the maximum batch size.
    ; Preconditions:
    ;   - The mill's current process is set to produce sugar of brand ?b.
    ;   - The mill is available.
    ;   - The sugar can produced from mixed raw cane ?r1 ?r2
    ;   - The available raw cane is less than max-produce.
    ; Effects:
    ;   - Increases storage by the exact amount of raw cane available.
    ;   - Sets the raw cane resource at the mill to zero.
    ;   - Marks the mill as unavailable (busy).
    ;   - Increases the mill's total cost by 4 times the process cost.
    (:action produce_sugar_from_mixed_raw_resource
		:parameters (?r1 ?r2 - raw-cane ?m - mill ?b - brand)
		:precondition (and 
				(current-brand-process ?m ?b)
				(available ?m)
                (mixed-raw-cane-production ?r1 ?r2 ?b)
			    (< (has-resource ?r1 ?m)(max-produce ?m))
                (< (has-resource ?r2 ?m)(max-produce ?m))
                (> (has-resource ?r1 ?m)0)
                (> (has-resource ?r2 ?m)0)
                (< (has-resource ?r2 ?m)(has-resource ?r1 ?m))
		     	     )
		:effect	     (and
				(increase (in-storage ?m ?b)(has-resource ?r2 ?m))
                (decrease (has-resource ?r1 ?m)(has-resource ?r2 ?m))
				(assign (has-resource ?r2 ?m)0)
				(not(available ?m))
				(increase (mills-total-cost) (* 4 (process-cost ?m)))
		     	     )
	)


    ; Action: setting-machine
    ; This action resets the mill to an available state after production.
    ; Preconditions:
    ;   - The mill is not currently available (it is busy).
    ; Effects:
    ;   - Makes the mill available for the next production action.
    (:action setting-machine
		:parameters (?m - mill)
		:precondition (and
				 (not (available ?m))
			      )
		:effect	      (and
				 (available ?m)
			      )
	)


    ; Action: switch-production-process
    ; This action allows a mill to change its production process from one sugar brand to another.
    ; Preconditions:
    ;   - The mill's current production process is set to produce brand ?b1.
    ;   - The mill is capable of producing brand ?b2.
    ;   - A process change from ?b1 to ?b2 is allowed.
    ;   - There is at least one production process change available (max-changing > 0).
    ; Effects:
    ;   - Updates the mill’s production process to brand ?b2.
    ;   - Removes the current production process for brand ?b1.
    ;   - Decreases the available production process changes by 1.
    (:action switch-production-process
        :parameters (?m - mill ?b1 - brand ?b2 - brand)
        :precondition (and
            (current-brand-process ?m ?b1)
            (can-produce ?m ?b2)
            (change-brand-process ?b1 ?b2)
            (> (max-changing ?m) 0)
        )
        :effect (and
            (current-brand-process ?m ?b2)
            (not (current-brand-process ?m ?b1))
            (decrease (max-changing ?m) 1)
        )
    )

    ; Action: order-sugar-cane
    ; This action is used when a mill has run out of raw cane and needs to place an order to acquire more.
    ; Preconditions:
    ;   - The quantity of raw cane available at the mill is exactly 0.
    ; Effects:
    ;   - Places an order for raw cane at the mill.
    (:action order-sugar-cane
        :parameters (?r - raw-cane ?m - mill)
        :precondition (and
            (= (has-resource ?r ?m) 0)   
        )
        :effect (and
            (place-order ?r ?m)         
        )
    )



    ; Action: harvest-cane-mill
    ; This action represents harvesting raw cane from a farm-field and transferring it directly to a mill.
    ; Preconditions:
    ;   - The farm-field ?f is near the mill ?m.
    ;   - The farm-field ?f is capable of harvesting raw cane of type ?r.
    ;   - The mill ?m has placed an order for raw cane ?r.
    ;   - The total amount of cane available in the field is at least equal to the cane yield for one harvest.
    ; Effects:
    ;   - Decreases the total cane available in the field by the cane yield.
    ;   - Increases the amount of raw cane available at the mill by the cane yield.
    ;   - Fulfills the order by removing the placed order.
    ;   - Increases the mill's total cost proportionally to the amount of cane harvested.
    (:action harvest-cane-mill
        :parameters (?f - farm-field ?r - raw-cane ?m - mill)
        :precondition (and
            (farmfield-near-mill ?f ?m)                   ; The field is near the mill.
            (farmfield-harvest-cane ?f ?r)                 ; The field can harvest raw cane ?r.
            (place-order ?r ?m)                           ; An order for raw cane is placed at the mill.
            (>= (total-canes-field ?f) (cane-yield ?f))     ; The field has at least the required yield available.
        )
        :effect (and
            (decrease (total-canes-field ?f) (cane-yield ?f))   ; Decrease the available cane in the field by the yield.
            (increase (has-resource ?r ?m) (cane-yield ?f))       ; Increase the mill's raw cane resource by the yield.
            (not (place-order ?r ?m))                           ; Remove the order after harvesting.
            (increase (mills-total-cost) (cane-yield ?f))        ; Increase the mill's cost proportional to the yield.
        )
    )

   ; Action: harvest-cane-truck
    ; This action represents harvesting raw cane from a farm-field and loading it directly onto a truck.
    ; Preconditions:
    ;   - The farm-field ?f is capable of harvesting raw cane ?r.
    ;   - The truck ?t is currently at the farm-field ?f.
    ;   - The truck has enough remaining capacity to accommodate the cane yield from the field.
    ;   - The farm-field has at least the required cane yield available.
    ; Effects:
    ;   - Decreases the total cane available in the field by the cane yield.
    ;   - Decreases the truck's remaining capacity by the same yield.
    ;   - Increases the amount of raw cane loaded on the truck by the cane yield.
    ;   - Increases the handling cost proportionally to the amount of cane harvested.
    (:action harvest-cane-truck
        :parameters (?f - farm-field ?r - raw-cane ?t - truck)
        :precondition (and
            (farmfield-harvest-cane ?f ?r)                 ; The field is capable of harvesting raw cane ?r.
            (at-location ?t ?f)                             ; The truck is at the farm-field.
            (>= (truck-remaining-capacity ?t) (cane-yield ?f)) ; The truck has sufficient remaining capacity.
            (>= (total-canes-field ?f) (cane-yield ?f))       ; The field has at least the required cane yield available.
        )
        :effect (and
            (decrease (total-canes-field ?f) (cane-yield ?f)) ; Reduce the available cane in the field by the yield.
            (decrease (truck-remaining-capacity ?t) (cane-yield ?f)) ; Reduce the truck's remaining capacity by the yield.
            (increase (truck-load-sugar ?r ?t) (cane-yield ?f)) ; Increase the truck's load by the cane yield.
            (increase (handle-cost) (cane-yield ?f))          ; Increase the handling cost proportionally to the harvested yield.
        )
    )



	
    ; Action: transfer-raw-cane-between-mills
    ; This action transfers raw cane from one mill (?m2) to another (?m1) when an order is placed.
    ; Preconditions:
    ;   - Mill ?m1 has placed an order for raw cane ?r.
    ;   - Mills ?m1 and ?m2 are connected.
    ;   - Mill ?m2 has at least one unit of raw cane ?r available.
    ; Effects:
    ;   - Increases the raw cane available at mill ?m1 by 1 unit.
    ;   - Decreases the raw cane available at mill ?m2 by 1 unit.
    ;   - Fulfills the order by removing it from mill ?m1.
    ;   - Decreases the inventory cost by 1 unit, reflecting cost savings in the transfer.
    (:action transfer-raw-cane-between-mills
        :parameters (?r - raw-cane ?m1 ?m2 - mill)
        :precondition (and
            (place-order ?r ?m1)
            (connected-mills ?m1 ?m2)
            (> (has-resource ?r ?m2) 0)
        )
        :effect (and
            (increase (has-resource ?r ?m1) 1)
            (decrease (has-resource ?r ?m2) 1)
            (not (place-order ?r ?m1))
        )
    )


    ; Action: aggregate-raw-cane-between-mills
    ; This action aggregates all available raw cane from mill ?m2 to mill ?m1 when an order is placed.
    ; Preconditions:
    ;   - Mill ?m1 has placed an order for raw cane ?r.
    ;   - Mills ?m1 and ?m2 are connected.
    ;   - Mill ?m2 has a positive quantity of raw cane available.
    ; Effects:
    ;   - Increases the raw cane available at mill ?m1 by the entire amount available at mill ?m2.
    ;   - Sets the raw cane resource at mill ?m2 to zero.
    ;   - Fulfills the order for raw cane at mill ?m1.
    (:action aggregate-raw-cane-between-mills
        :parameters (?r - raw-cane ?m1 ?m2 - mill)
        :precondition (and
            (place-order ?r ?m1)
            (connected-mills ?m1 ?m2)
            (> (has-resource ?r ?m2) 0)
        )
        :effect (and
            (increase (has-resource ?r ?m1) (has-resource ?r ?m2))
            (assign (has-resource ?r ?m2) 0)
            (not (place-order ?r ?m1))
        )
    )


    ; Action: load_truck_crane
    ; This action uses a crane to load sugar of a given brand from storage at a location onto a truck.
    ; Preconditions:
    ;   - The truck ?t and crane ?c are both located at location ?l.
    ;   - There is enough sugar of brand ?b in storage at location ?l to fill the crane's capacity.
    ;   - The truck has enough remaining capacity to accommodate the load equal to the crane's capacity.
    ;   - The crane's maintenance time is greater than 0, indicating it is operational but may be nearing maintenance.
    ; Effects:
    ;   - Decreases the sugar in storage at location ?l by the crane's capacity.
    ;   - Decreases the truck's remaining capacity by the crane's capacity.
    ;   - Increases the sugar loaded onto the truck by the crane's capacity.
    ;   - Increases the handling cost by an amount proportional to the crane's capacity.
    ;   - Decreases the crane's maintenance time by 1, representing usage wear.
    (:action load_truck_crane
        :parameters (?b - brand ?t - truck ?l - location ?c - crane)
        :precondition (and 
            (at-location ?t ?l)                                  ; The truck is at location ?l.
            (at-location ?c ?l)                                  ; The crane is at location ?l.
            (>= (in-storage ?l ?b) (capacity ?c))                ; There is enough sugar in storage to fill the crane's capacity.
            (>= (truck-remaining-capacity ?t) (capacity ?c))      ; The truck has sufficient remaining capacity.
            (> (maintenance-time ?c) 0)                         ; The crane has positive maintenance time available.
        )
        :effect (and
            (decrease (in-storage ?l ?b) (capacity ?c))         ; Remove sugar from storage equal to the crane's capacity.
            (decrease (truck-remaining-capacity ?t) (capacity ?c)) ; Reduce the truck's available capacity by the crane's capacity.
            (increase (truck-load-sugar ?b ?t) (capacity ?c))      ; Increase the sugar loaded on the truck by the crane's capacity.
            (increase (handling-cost) (capacity ?c))             ; Increase handling cost proportional to the crane's capacity.
            (decrease (maintenance-time ?c) 1)                   ; Decrease the crane's maintenance time by 1 unit.
        )
    )

    ; Action: load-truck-manual
    ; This action represents manually loading sugar from storage onto a truck.
    ; Preconditions:
    ;   - The truck ?t is at location ?l.
    ;   - There is at least one unit of sugar of brand ?b in storage at location ?l.
    ;   - The truck has at least one unit of remaining capacity.
    ; Effects:
    ;   - Decreases the sugar in storage at location ?l by 1 unit.
    ;   - Decreases the truck's remaining capacity by 1 unit.
    ;   - Increases the sugar loaded onto the truck by 1 unit.
    ;   - Increases the handling cost by 1 unit.
        (:action load-truck-manual
        :parameters (?b - brand ?t - truck ?l - location)
        :precondition (and 
            (at-location ?t ?l)
            (> (in-storage ?l ?b) 0)
            (> (truck-remaining-capacity ?t) 0)
        )
        :effect (and
            (decrease (in-storage ?l ?b) 1)
            (decrease (truck-remaining-capacity ?t) 1)
            (increase (truck-load-sugar ?b ?t) 1)
            (increase (handling-cost) 1)
        )
    )


    ; Action: maintainence-crane
    ; This action represents performing maintenance on a crane to restore its maintenance time.
    ; Preconditions:
    ;   - The crane ?c is at location ?l.
    ;   - The crane's maintenance time is 0, indicating that maintenance is required.
    ; Effects:
    ;   - Increases the crane's maintenance time by 5 units, effectively resetting its maintenance timer.
    (:action maintainence-crane
        :parameters (?c - crane ?l - location)
        :precondition (and
            (at-location ?c ?l)
            (= (maintenance-time ?c) 0)
        )
        :effect (and
            (increase (maintenance-time ?c) 5)
        )
    )
			
	
    ; Action: drive_truck
    ; This action models the movement of a truck from one location to another.
    ; Preconditions:
    ;   - The truck ?t is currently at the starting location ?l1.
    ;   - There is a connection between the starting location ?l1 and the destination location ?l2.
    ; Effects:
    ;   - The truck ?t moves to the destination location ?l2.
    ;   - The truck is no longer at the starting location ?l1.
    (:action drive_truck
    `    :parameters (?t - truck ?l1 ?l2 - location)
        :precondition (and
            (at-location ?t ?l1)
            (connected ?l1 ?l2)
        )
        :effect (and
            (at-location ?t ?l2)
            (not (at-location ?t ?l1))
        )
    )

    ; Action: unload_truck
    ; This action unloads one unit of sugar of a specified brand from a truck at a given location.
    ; Preconditions:
    ;   - The truck ?t is located at the specified location ?l.
    ;   - There is at least one unit of sugar of brand ?b loaded on the truck.
    ; Effects:
    ;   - Increases the storage of sugar of brand ?b at location ?l by one unit.
    ;   - Decreases the sugar loaded on the truck by one unit (Note: predicate should match truck-load or truck-load-sugar consistently).
    ;   - Increases the truck's remaining capacity by one unit.
    (:action unload_truck
        :parameters (?b - brand ?t - truck ?l - location)
        :precondition (and 
            (at-location ?t ?l)
            (> (truck-load-sugar ?b ?t) 0)
        )
        :effect (and 
            (increase (in-storage ?l ?b) 1)
            (decrease (in-truck-sugar ?b ?t) 1)
            (increase (truck-remaining-capacity ?t) 1)
        )
    )

    ; Action: unload_truck_max
    ; This action unloads all sugar of a specified brand from a truck at a given location.
    ; Preconditions:
    ;   - The truck ?t is located at the specified location ?l.
    ;   - There is at least one unit of sugar of brand ?b loaded on the truck.
    ; Effects:
    ;   - Increases the storage of sugar of brand ?b at location ?l by the total amount loaded on the truck.
    ;   - Sets the sugar loaded on the truck for brand ?b to zero.
    ;   - Increases the truck's remaining capacity by the amount that was unloaded.
    (:action unload_truck_max
        :parameters (?b - brand ?t - truck ?l - location)
        :precondition (and 
            (at-location ?t ?l)
            (> (truck-load-sugar ?b ?t) 0)
        )
        :effect (and 
            (increase (in-storage ?l ?b) (truck-load-sugar ?b ?t))
            (assign (in-truck-sugar ?b ?t) 0)
            (increase (truck-remaining-capacity ?t) (truck-load-sugar ?b ?t))
        )
    )


)



