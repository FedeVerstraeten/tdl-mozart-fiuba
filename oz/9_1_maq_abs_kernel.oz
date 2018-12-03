local MyMap L Lout A B1 B2 C F in
    MyMap = proc {$ Xs F R}
        case Xs of nil then
            R = nil
        else
        case Xs of X|Xr then
            local A T in
                {F X A}
                {MyMap Xr F T}
                R = A|T
            end
        else
            skip
        end	
      end
    end
    
    L = [1 2 3 4]

    F = proc {$ X R1} 
        local P in 
            P = proc {$ R1}
                R1 = X*X
            end
            {ByNeed P R1}
        end 
    end

    {MyMap L F Lout}    
    A = Lout.1
    B1 = Lout.2
    B2 = B1.1
    C = A + B2
    {Browse C}
end
