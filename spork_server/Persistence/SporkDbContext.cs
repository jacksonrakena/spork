using Microsoft.EntityFrameworkCore;

namespace Spork.Server.Persistence;

public class SporkDbContext : DbContext
{
    public DbSet<ParliamentMember> Members { get; set; }
    public DbSet<ParliamentMemberTerm> Terms { get; set; }
    public DbSet<ParliamentElectorate> Electorates { get; set; }

    public SporkDbContext(DbContextOptions<SporkDbContext> options) : base(options)
    {
    }
}