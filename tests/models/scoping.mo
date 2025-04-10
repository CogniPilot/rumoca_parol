package scoping
    import my_import;
    model Integrator
        Real x;
    equation
        der(x) = 1;
    end Integrator;
end scoping;