using Microsoft.AspNetCore.Mvc;
using Microsoft.EntityFrameworkCore;
using Spork.Server.Persistence;

namespace Spork.Server.Controllers.v1.Parliament;

[ApiController]
[Route("api/v1/parliament")]
public class ParliamentController
{
    private readonly SporkDbContext _database;
    public ParliamentController(SporkDbContext context)
    {
        _database = context;
    }
    
    [HttpGet("members")]
    public async Task<IEnumerable<ParliamentMember>> GetAllMembers()
    {
        return await _database.Members.ToListAsync();
    }
}