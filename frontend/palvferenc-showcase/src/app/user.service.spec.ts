import { HttpClientTestingModule, HttpTestingController } from '@angular/common/http/testing';
import { TestBed } from '@angular/core/testing';

import { HttpClient} from '@angular/common/http';

import { UserService } from './user.service';
import { User } from './model/user';
import { environment } from 'src/environments/environment';

describe('UserService', () => {
  let httpClient: HttpClient;
  let httpTestingController: HttpTestingController;
  let userService: UserService;

  beforeEach(() => { TestBed.configureTestingModule({
    imports: [ HttpClientTestingModule ],
    providers: [
      UserService,
    ]
  })

  httpClient = TestBed.get(HttpClient);
  httpTestingController = TestBed.get(HttpTestingController);
  userService = TestBed.get(UserService);

  });

  afterEach(() => {
    // After every test, assert that there are no more pending requests.
    httpTestingController.verify();
  });

  describe('#getUsers', () => {
    let expectedUsers: User[];

    beforeEach(() => {
      userService = TestBed.get(UserService);
      expectedUsers = [
        { id: 1, name: 'User1' },
        { id: 2, name: 'User2' },
       ] as User[];
    });

    it('should return expected users (called once)', () => {

      userService.getUsers().subscribe(
        users => expect(users).toEqual(expectedUsers, 'should return expected users'),
        fail
      );

      // HeroService should have made one request to GET heroes from expected URL
      const req = httpTestingController.expectOne(`${environment.servicesBaseUrl}/user`);
      expect(req.request.method).toEqual('GET');

      // Respond with the mock heroes
      req.flush(expectedUsers);
    });
  });
});
